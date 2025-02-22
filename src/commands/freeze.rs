use crate::{
    exit,
    helpers::{get_config_dir, Exit},
    projects::{Project, ProjectSetup},
    templates::Window as TemplateWindow,
};
use itertools::Itertools;
use std::{
    collections::BTreeMap,
    fmt,
    path::{Path, PathBuf},
};
use tmux_interface::{Tmux, TmuxCommand};

const SEPERATOR: &str = "\t";

#[derive(Debug)]
struct Window {
    name: String,
    layout: String,
    panes: Vec<PathBuf>,
}

impl Window {
    #[allow(clippy::wrong_self_convention)]
    fn to_template_window(self, most_used_path: &Path) -> TemplateWindow {
        let panes = self
            .panes
            .into_iter()
            .map(|dir| {
                if dir == most_used_path {
                    String::default()
                } else {
                    let escaped_name = dir
                        .display()
                        .to_string()
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"");
                    format!("cd {}", escaped_name)
                }
            })
            .collect();

        TemplateWindow {
            panes,
            name: Some(self.name),
            layout: Some(self.layout),
        }
    }
}

#[derive(Debug)]
enum ActiveTmuxInstance {
    Window {
        name: String,
        layout: String,
        index: usize,
        session_name: String,
    },
    Pane {
        window_index: usize,
        cwd: PathBuf,
        session_name: String,
    },
}

#[derive(Debug)]
enum TmuxParseError {
    NoPrefix,
    UndefinedPrefix(String),
    MissingData(usize),
    NoNumber(String),
}

impl fmt::Display for TmuxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPrefix => write!(f, ""),
            Self::UndefinedPrefix(prefix) => write!(f, "The prefix {prefix} isn't defined"),
            Self::MissingData(0) => write!(f, "The first entry is missing"),
            Self::MissingData(1) => write!(f, "The second entry is missing"),
            Self::MissingData(index) => write!(f, "The {}th entry is missing", index + 1),
            Self::NoNumber(string) => write!(f, "{string} isn't a number"),
        }
    }
}

impl std::error::Error for TmuxParseError {}

impl TryFrom<&str> for ActiveTmuxInstance {
    type Error = TmuxParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(SEPERATOR);
        let indicator = parts.next().ok_or(TmuxParseError::NoPrefix)?;
        match indicator {
            "window:" => {
                let name = parts.next().ok_or(TmuxParseError::MissingData(0))?.into();
                let layout = parts.next().ok_or(TmuxParseError::MissingData(1))?.into();
                let index: String = parts.next().ok_or(TmuxParseError::MissingData(2))?.into();
                let index = index.parse().map_err(|_| TmuxParseError::NoNumber(index))?;
                let session_name = parts.next().ok_or(TmuxParseError::MissingData(3))?.into();
                Ok(Self::Window {
                    name,
                    layout,
                    index,
                    session_name,
                })
            }
            "pane:" => {
                let window_index: String =
                    parts.next().ok_or(TmuxParseError::MissingData(0))?.into();
                let window_index = window_index
                    .parse()
                    .map_err(|_| TmuxParseError::NoNumber(window_index))?;
                let cwd = parts.next().ok_or(TmuxParseError::MissingData(2))?;
                let cwd = PathBuf::from(cwd);
                let session_name = parts.next().ok_or(TmuxParseError::MissingData(3))?.into();
                Ok(Self::Pane {
                    window_index,
                    cwd,
                    session_name,
                })
            }
            str => Err(TmuxParseError::UndefinedPrefix(str.into())),
        }
    }
}

fn current_windows() -> Vec<Window> {
    // as far i know the name/layout/index can't include a tab
    let window_cmd = TmuxCommand::list_windows()
        .format("window:\t#{window_name}\t#{window_layout}\t#{window_index}\t#{session_name}");
    let panes_cmd = TmuxCommand::list_panes()
        .all()
        .format("pane:\t#{window_index}\t#{pane_current_path}\t#{session_name}");

    let output = Tmux::new()
        .add_command(window_cmd)
        .add_command(panes_cmd)
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout()).ok())
        .exit(1, "Can't receive current tmux session setup");

    let tmux_panes_windows =
        output
            .lines()
            .filter_map(|line| match ActiveTmuxInstance::try_from(line) {
                Ok(data) => Some(data),
                Err(err) => {
                    eprintln!("Parsing of one pane or window failed: {err}");
                    None
                }
            });
    let window_map = tmux_panes_windows.fold(BTreeMap::new(), |mut windows_map, instance| {
        match instance {
            ActiveTmuxInstance::Window {
                name,
                layout,
                index,
                session_name,
            } => {
                windows_map.entry((index, session_name)).or_insert(Window {
                    name,
                    layout,
                    panes: Vec::new(),
                });
            }
            ActiveTmuxInstance::Pane {
                window_index,
                cwd,
                session_name,
            } => {
                if let Some(window) = windows_map.get_mut(&(window_index, session_name)) {
                    window.panes.push(cwd);
                }
            }
        }
        windows_map
    });
    window_map.into_values().collect()
}

trait ConfigWriter {
    fn write(&self, project: Project) -> Result<Option<String>, String>;
}

struct File(PathBuf);

impl File {
    fn try_new<N: fmt::Display>(name: N, force: bool) -> Result<Self, String> {
        let name = name.to_string();
        if name.contains('/') {
            return Err(String::from("File name can't contain /"));
        }

        let file_name = if name.ends_with(".yaml") {
            name
        } else {
            format!("{name}.yaml")
        };

        let file_path = get_config_dir().join("projects/").join(file_name);

        if file_path.exists() && !force {
            Err(format!(
                "A file {} alreay exists, use -f to overwrite it",
                file_path.display()
            ))
        } else {
            Ok(Self(file_path))
        }
    }
}

impl ConfigWriter for File {
    fn write(&self, project: Project) -> Result<Option<String>, String> {
        let yaml =
            serde_yaml::to_string(&project).map_err(|err| format!("Can't create yaml: {err}"))?;

        match std::fs::write(&self.0, yaml) {
            Ok(_) => Ok(Some(format!(
                "Froze configuration into {}",
                self.0.display()
            ))),
            Err(err) => Err(format!("Can't write file: {}", err)),
        }
    }
}

struct StdOut;

impl ConfigWriter for StdOut {
    fn write(&self, project: Project) -> Result<Option<String>, String> {
        let yaml =
            serde_yaml::to_string(&project).map_err(|err| format!("Can't create yaml: {err}"))?;

        println!("{yaml}");
        Ok(None)
    }
}

fn new_config_writer<T: fmt::Display>(
    stdout: bool,
    file_name: T,
    force: bool,
) -> Result<Box<dyn ConfigWriter>, String> {
    if stdout {
        Ok(Box::new(StdOut))
    } else {
        let file = File::try_new(file_name, force)?;
        Ok(Box::new(file))
    }
}

pub fn freeze_handler(name: Option<String>, force: bool, file_name: Option<String>, stdout: bool) {
    let windows = current_windows();
    let most_used_path = windows
        .iter()
        .flat_map(|window| &window.panes)
        .counts()
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .expect("There must be at least one pane")
        .0
        .clone();
    let most_used_path = most_used_path.as_path();
    let name = name
        .or_else(|| {
            most_used_path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| String::from("Unnamed Project"));

    let writer = match new_config_writer(stdout, file_name.as_ref().unwrap_or(&name), force) {
        Ok(f) => f,
        Err(err) => exit!(1, "{err}"),
    };

    let template_wins: Vec<_> = windows
        .into_iter()
        .map(|win| win.to_template_window(most_used_path))
        .collect();
    let proj = Project {
        name,
        root_dir: most_used_path.to_path_buf(),
        setup: ProjectSetup::Windows {
            windows: template_wins,
        },
    };

    match writer.write(proj) {
        Ok(Some(msg)) => println!("{msg}"),
        Ok(None) => {}
        Err(msg) => exit!(1, "{msg}"),
    }
}
