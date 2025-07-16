use thiserror::Error;

use crate::{
    helpers::{get_config_dir, Exit},
    widgets::table::Table,
};
use std::{
    collections::{hash_map::Keys, HashMap},
    fs,
    path::PathBuf,
};

#[derive(Debug, Default, Clone)]
pub struct Directories(HashMap<String, PathBuf>);

impl Directories {
    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.0.get(name)
    }

    pub fn names(&self) -> Keys<String, PathBuf> {
        self.0.keys()
    }
}

impl std::iter::IntoIterator for Directories {
    type Item = (String, PathBuf);
    type IntoIter = std::collections::hash_map::IntoIter<String, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Directories> for Table<String, String> {
    fn from(value: Directories) -> Self {
        value
            .into_iter()
            .map(|(name, path)| (name, path.display().to_string()))
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum ParseDirectoryError {
    #[error("A name for {} can't be determined", dir.display())]
    NoDirName { dir: PathBuf },
    #[error("the name {name} is associated with both {} and {}", values.0.display(), values.1.display())]
    DuplicateName {
        name: String,
        values: (PathBuf, PathBuf),
    },
}

pub fn parse_directory_config() -> Result<Directories, ParseDirectoryError> {
    let file_content = fs::read_to_string(get_config_dir().join("directories.yaml"))
        .exit(1, "Can't read directories config file");

    let mut hm = HashMap::new();

    for line in file_content.lines() {
        if line.starts_with('#') {
            // Comments
            continue;
        }

        let (name, dir) = line.split_once(':').map_or_else(
            || {
                let path = PathBuf::from(line.trim());
                let name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| ParseDirectoryError::NoDirName { dir: path.clone() })?;
                Ok((name.to_string(), path))
            },
            |(name, path)| Ok((name.trim().to_string(), PathBuf::from(path.trim()))),
        )?;

        // TODO: Remove cloning
        if let Some(prev) = hm.insert(name.clone(), dir.clone()) {
            return Err(ParseDirectoryError::DuplicateName {
                name: name.to_string(),
                values: (prev, dir),
            });
        }
    }

    Ok(Directories(hm))
}
