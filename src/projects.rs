use crate::{
    exit,
    helpers::{get_config_dir, Exit},
    templates::{parse_template_config, Window},
    widgets::table::Table,
};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub root_dir: PathBuf,
    pub setup: ProjectSetup,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ProjectSetup {
    Template(String),
    Windows(Vec<Window>),
}

impl From<ProjectSetup> for Vec<Window> {
    fn from(val: ProjectSetup) -> Self {
        match val {
            ProjectSetup::Template(template_name) => {
                let all_templates = parse_template_config();
                let template = all_templates
                    .into_iter()
                    .find(|t| t.name == template_name)
                    .unwrap_or_else(|| exit!(1, "Template {} could not be found", template_name));

                template.windows
            }
            ProjectSetup::Windows(windows) => windows,
        }
    }
}

impl From<ProjectSetup> for Table<String, String> {
    fn from(value: ProjectSetup) -> Self {
        let template_name = match &value {
            ProjectSetup::Template(template_name) => Some(template_name.clone()),
            ProjectSetup::Windows(_) => None,
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
            ProjectSetup::Windows(windows)
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

pub fn parse_project_config() -> Vec<Project> {
    let projects_content =
        fs::read_dir(get_config_dir().join("projects/")).exit(1, "Can't read template config");

    let projects_raw: Vec<_> = projects_content
        .filter_map(|x| x.ok())
        .filter(|x| x.path().is_file())
        .filter_map(|x| fs::read_to_string(x.path()).ok())
        .collect();

    projects_raw
        .iter()
        .filter_map(|x| serde_yaml::from_str::<Project>(x).ok())
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
                setup: ProjectSetup::Windows(vec![
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
                ])
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
