use crate::{
    helpers::{get_config_dir, Exit},
    widgets::table::Table,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub windows: Vec<Window>,
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub name: Option<String>,
    pub panes: Vec<String>,
}

impl Table<String, String> for Window {
    fn table(&self) -> (String, String) {
        let name = self.name.clone().unwrap_or("No name".to_string());

        (name, format!("{} Panes", self.panes.len()))
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
