use crate::helpers::get_config_dir;
use std::fs;
use std::io;
use std::path::PathBuf;

fn create_config_dir() -> io::Result<PathBuf> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

fn create_config_files(config_path: &PathBuf) -> io::Result<()> {
    fs::create_dir(config_path.join("projects"))?;
    fs::create_dir(config_path.join("templates"))?;
    fs::File::create(config_path.join("directories.yaml"))?;

    Ok(())
}

pub fn init_config() -> io::Result<PathBuf> {
    let config_path = create_config_dir()?;
    create_config_files(&config_path)?;

    Ok(config_path)
}
