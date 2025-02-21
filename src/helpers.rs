use crate::exit;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn get_config_dir() -> PathBuf {
    env::var("XDG_CONFIG_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".config")))
        .or_else(dirs::config_dir)
        .exit(1, "Could not find a config directory")
        .join("moxide")
}

fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let p = path.as_ref();

    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }

    let mut home_dir = dirs::home_dir()?;
    if home_dir == Path::new("/") {
        // Corner case: `h` root directory;
        // don't prepend extra `/`, just drop the tilde.
        Some(p.strip_prefix("~").unwrap().to_path_buf())
    } else {
        home_dir.push(p.strip_prefix("~/").unwrap());
        Some(home_dir)
    }
}

pub fn absolute_path(path: &Path) -> std::io::Result<PathBuf> {
    let expanded = expand_tilde(path).unwrap_or_else(|| path.to_path_buf());
    fs::canonicalize(expanded)
}

pub trait Exit<T> {
    fn exit<M: std::fmt::Display>(self, code: i32, msg: M) -> T;
}

impl<T, E> Exit<T> for Result<T, E> {
    fn exit<M: std::fmt::Display>(self, code: i32, msg: M) -> T {
        self.unwrap_or_else(|_| exit!(code, "{}", msg))
    }
}
impl<T> Exit<T> for Option<T> {
    fn exit<M: std::fmt::Display>(self, code: i32, msg: M) -> T {
        self.unwrap_or_else(|| exit!(code, "{}", msg))
    }
}

pub fn runs_in_tmux() -> bool {
    env::var("TMUX").is_ok()
}

pub fn dir_name(path: &Path) -> String {
    path.file_name()
        .and_then(|os_string| os_string.to_str())
        .map(|str| str.to_string())
        .unwrap_or_default()
}

pub fn format_name(user_fmt: &Option<String>, name: &str) -> String {
    user_fmt
        .as_ref()
        .map_or_else(|| name.to_string(), |fmt| fmt.replace("{}", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            expand_tilde(PathBuf::from("~/foo")).unwrap(),
            PathBuf::from(format!("{}/foo", home))
        );
    }
}
