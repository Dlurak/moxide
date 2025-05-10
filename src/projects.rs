use crate::{
    exit,
    helpers::{get_config_dir, Exit},
    templates::{find_template, Window},
    widgets::table::Table,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub root_dir: PathBuf,
    #[serde(flatten)]
    pub setup: ProjectSetup,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ProjectSetup {
    Template(String),
    Windows { windows: Vec<Window> },
}

impl From<ProjectSetup> for Vec<Window> {
    fn from(val: ProjectSetup) -> Self {
        match val {
            ProjectSetup::Template(template_name) => {
                let template = find_template(&template_name)
                    .unwrap_or_else(|| exit!(1, "Template {} could not be found", template_name));

                template.windows
            }
            ProjectSetup::Windows { windows } => windows,
        }
    }
}

impl From<ProjectSetup> for Table<String, String> {
    fn from(value: ProjectSetup) -> Self {
        let template_name = match &value {
            ProjectSetup::Template(template_name) => Some(template_name.clone()),
            ProjectSetup::Windows { .. } => None,
        };
        let windows: Vec<Window> = value.into();
        let windows: Vec<&Window> = windows.iter().collect();

        let mut rows = Self::new(vec![(
            "Template".to_string(),
            template_name.unwrap_or_else(|| "None".to_string()),
        )]);
        rows.extend_table(Self::from_iter(windows));

        rows
    }
}

impl<'de> Deserialize<'de> for Project {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawProject {
            name: String,
            root_dir: PathBuf,
            template: Option<String>,
            windows: Option<Vec<Window>>,
        }

        let raw = RawProject::deserialize(deserializer)?;

        let setup = if let Some(template) = raw.template {
            ProjectSetup::Template(template)
        } else if let Some(windows) = raw.windows {
            ProjectSetup::Windows { windows }
        } else {
            return Err(serde::de::Error::custom(
                "Expected either template or windows",
            ));
        };

        Ok(Self {
            name: raw.name,
            root_dir: raw.root_dir,
            setup,
        })
    }
}

pub fn find_project(name: &str) -> Option<Project> {
    let projects_dir = get_config_dir().join("projects/");
    let file_path = projects_dir.join(format!("{name}.yaml"));
    let is_valid_path = file_path.exists() && file_path.is_file();
    let matching_path = is_valid_path.then_some(file_path);

    let matching_project = matching_path.and_then(|path| {
        let content = fs::read_to_string(&path).ok()?;
        let project = serde_yaml::from_str::<Project>(&content).ok()?;
        (project.name == name).then_some(project)
    });

    if matching_project.is_some() {
        return matching_project;
    }

    fs::read_dir(&projects_dir)
        .exit(1, "Can't read template config")
        .find_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            let content = fs::read_to_string(&path).ok()?;
            let project = serde_yaml::from_str::<Project>(&content).ok()?;
            (project.name == name).then_some(project)
        })
}

pub fn parse_project_config() -> Vec<Project> {
    let projects_dir = get_config_dir().join("projects/");
    let projects_content = fs::read_dir(&projects_dir).exit(1, "Can't read template config");

    projects_content
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            let content = fs::read_to_string(&path).ok()?;
            serde_yaml::from_str::<Project>(&content).ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let project = serde_yaml::from_str::<Project>(
            "name: OsmApp

root_dir: ~/GitHub/osmapp/
windows:
  - name:  Neovim
    panes:
      - nvim
  - name: Server
    panes:
      - yarn run dev",
        )
        .unwrap();

        assert_eq!(
            project,
            Project {
                name: "OsmApp".to_string(),
                root_dir: PathBuf::from("~/GitHub/osmapp"),
                setup: ProjectSetup::Windows {
                    windows: vec![
                        Window {
                            name: Some(" Neovim".to_string()),
                            panes: vec!["nvim".to_string()],
                            layout: None,
                        },
                        Window {
                            name: Some("Server".to_string()),
                            panes: vec!["yarn run dev".to_string()],
                            layout: None,
                        }
                    ]
                }
            }
        );

        let project = serde_yaml::from_str::<Project>(
            "name: Dlool

root_dir: ~/SoftwareDevelopment/web/Dlool/dlool_frontend_v2/
template: Svelte",
        )
        .unwrap();

        assert_eq!(
            project,
            Project {
                name: "Dlool".to_string(),
                root_dir: PathBuf::from("~/SoftwareDevelopment/web/Dlool/dlool_frontend_v2/"),
                setup: ProjectSetup::Template("Svelte".to_string())
            }
        );
    }
}
