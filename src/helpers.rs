use crate::exit;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_config_dir() -> std::path::PathBuf {
    match dirs::config_dir() {
        Some(path) => path.join("muxmate"),
        None => exit!(1, "Could  not find a config directory"),
    }
}

fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let p = path.as_ref();

    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

pub fn absolute_path(path: &Path) -> std::io::Result<PathBuf> {
    let expanded = expand_tilde(path).unwrap_or(path.to_path_buf());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            expand_tilde(&PathBuf::from("~/foo")).unwrap(),
            PathBuf::from(format!("{}/foo", home))
        );
    }
}
