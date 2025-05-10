use crate::{
    helpers::{get_config_dir, Exit},
    widgets::table::Table,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tmux_interface::{Tmux, TmuxCommand};

#[derive(Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub hidden: Option<bool>,
    pub windows: Vec<Window>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Window {
    pub name: Option<String>,
    pub layout: Option<String>,
    pub panes: Vec<String>,
}

impl From<&Window> for Table<String, String> {
    fn from(value: &Window) -> Self {
        let name = value.name.clone().unwrap_or_else(|| "No name".to_string());

        Self::from((name, format!("{} Panes", value.panes.len())))
    }
}

pub fn find_template(name: &str) -> Option<Template> {
    let templates_dir = get_config_dir().join("templates/");
    let file_path = templates_dir.join(format!("{name}.yaml"));
    let is_valid_path = file_path.exists() && file_path.is_file();
    let matching_path = is_valid_path.then_some(file_path);

    let matching_template = matching_path.and_then(|path| {
        let content = fs::read_to_string(&path).ok()?;
        let template = serde_yaml::from_str::<Template>(&content).ok()?;
        (template.name == name).then_some(template)
    });

    if matching_template.is_some() {
        return matching_template;
    }

    fs::read_dir(&templates_dir)
        .exit(1, "Can't read template config")
        .find_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            let content = fs::read_to_string(&path).ok()?;
            let template = serde_yaml::from_str::<Template>(&content).ok()?;
            (template.name == name).then_some(template)
        })
}

pub fn parse_template_config() -> Vec<Template> {
    let templates_content =
        fs::read_dir(get_config_dir().join("templates/")).exit(1, "Can't read template config");

    templates_content
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            let content = fs::read_to_string(path).ok()?;
            serde_yaml::from_str::<Template>(&content).ok()
        })
        .collect()
}

pub fn apply_windows<'a>(
    tmux: Tmux<'a>,
    windows: &'a [Window],
    dir: Option<&'a PathBuf>,
) -> Tmux<'a> {
    let enumerated = windows.iter().enumerate();
    enumerated.fold(tmux, |tmux, (window_idx, window)| {
        let cmd = build_tmux_command(window_idx, window, dir);
        let tmux = add_panes_to_tmux(tmux.add_command(cmd), &window.panes, dir);

        match window.layout.as_ref() {
            Some(layout) => {
                let layout_cmd = TmuxCommand::select_layout().layout_name(layout);
                tmux.add_command(layout_cmd)
            }
            None => tmux,
        }
    })
}

fn add_panes_to_tmux<'a>(tmux: Tmux<'a>, panes: &[String], dir: Option<&'a PathBuf>) -> Tmux<'a> {
    let enumerated = panes.iter().enumerate();

    enumerated.fold(tmux, |tmux, (pane_idx, command)| {
        let cmd: TmuxCommand = match (pane_idx, dir) {
            (0, _) => TmuxCommand::new(),
            (_, Some(d)) => TmuxCommand::split_window()
                .start_directory(d.to_string_lossy())
                .into(),
            (_, None) => TmuxCommand::split_window().into(),
        };

        tmux.add_command(cmd)
            .add_command(TmuxCommand::send_keys().key(format!("{command}\r")))
    })
}

fn build_tmux_command<'a>(
    window_idx: usize,
    window: &'a Window,
    dir: Option<&'a PathBuf>,
) -> TmuxCommand<'a> {
    if window_idx == 0 {
        window
            .name
            .as_ref()
            .map(|name| TmuxCommand::rename_window().new_name(name).into())
            .unwrap_or_default()
    } else {
        let name = window.name.as_ref();
        let new_win = name.map_or_else(TmuxCommand::new_window, |name| {
            TmuxCommand::new_window().window_name(name)
        });
        match dir {
            Some(d) => new_win.start_directory(d.to_string_lossy()).into(),
            None => new_win.into(),
        }
    }
}
