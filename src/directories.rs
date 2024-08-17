use crate::{
    helpers::{get_config_dir, Exit},
    widgets::table::Table,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt, fs};

#[derive(Debug, Deserialize)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Directory {
    pub fn get_name(&self) -> &str {
        match &self.name {
            Some(name) => name,
            None => self
                .path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or(""),
        }
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{} {}", self.path.display(), name),
            None => write!(f, "{}", self.path.display()),
        }
    }
}

impl Table<String, String> for Directory {
    fn table(&self) -> (String, String) {
        let first_col = match (&self.icon, &self.name) {
            (Some(icon), Some(name)) => format!("{} {}", icon, name),
            (Some(icon), None) => icon.clone(),
            (None, Some(name)) => name.clone(),
            (None, None) => "No name".to_string(),
        };

        (first_col, self.path.display().to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct Categories {
    #[serde(flatten)]
    pub categories: HashMap<String, Vec<Directory>>,
}

pub fn parse_directory_config() -> Categories {
    let yaml_content = fs::read_to_string(get_config_dir().join("directories.yaml"))
        .exit(1, "Can't read directories config");
    let categories: Categories = serde_yaml::from_str(&yaml_content)
        .exit(1, "Can't parse the directories config, please correct it");

    categories
}
