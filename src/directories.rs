use crate::helpers::get_config_dir;
use crate::widgets::table::Table;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub name: Option<String>,
    pub icon: Option<String>,
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

#[derive(Debug)]
pub enum DirectoryParseError {
    ParseError(serde_yaml::Error),
    IoError(io::Error),
}

impl From<io::Error> for DirectoryParseError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_yaml::Error> for DirectoryParseError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::ParseError(value)
    }
}

pub fn parse_directory_config() -> Result<Categories, DirectoryParseError> {
    let yaml_content = fs::read_to_string(get_config_dir().join("directories.yaml"))?;
    let categories: Categories = serde_yaml::from_str(&yaml_content)?;

    Ok(categories)
}
