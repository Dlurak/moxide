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

pub fn parse_template_config() -> Vec<Template> {
    let templates_content =
        fs::read_dir(get_config_dir().join("templates/")).exit(1, "Can't read template config");

    let templates_raw: Vec<_> = templates_content
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_file())
        .filter_map(|x| fs::read_to_string(x.path()).ok())
        .collect();

    templates_raw
        .iter()
        .filter_map(|x| serde_yaml::from_str::<Template>(x).ok())
        .collect()
}

pub fn apply_windows<'a>(
    tmux: Tmux<'a>,
    windows: &'a [Window],
    dir: &'a Option<PathBuf>,
) -> Tmux<'a> {
    let enumerated = windows.iter().enumerate();
    enumerated.fold(tmux, |tmux, (window_idx, window)| {
        let cmd = build_tmux_command(window_idx, window, dir);

        let layout_cmd: TmuxCommand = window.layout.as_ref().map_or_else(|| TmuxCommand::select_layout().into(), |layout| TmuxCommand::select_layout().layout_name(layout).into());

        let tmux = tmux.add_command(cmd);
        add_panes_to_tmux(tmux, &window.panes, dir).add_command(layout_cmd)
    })
}

fn add_panes_to_tmux<'a>(tmux: Tmux<'a>, panes: &[String], dir: &'a Option<PathBuf>) -> Tmux<'a> {
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
            .add_command(TmuxCommand::send_keys().key(format!("{}\r", command)))
    })
}

fn build_tmux_command<'a>(
    window_idx: usize,
    window: &'a Window,
    dir: &'a Option<PathBuf>,
) -> TmuxCommand<'a> {
    if window_idx == 0 {
        window.name.as_ref().map_or_else(TmuxCommand::new, |name| {
            TmuxCommand::rename_window().new_name(name).into()
        })
    } else {
        let new_win = window
            .name
            .as_ref()
            .map_or_else(TmuxCommand::new_window, |name| {
                TmuxCommand::new_window().window_name(name)
            });
        match dir {
            Some(d) => new_win.start_directory(d.to_string_lossy()).into(),
            None => new_win.into(),
        }
    }
}
