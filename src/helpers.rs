pub fn get_config_dir() -> std::path::PathBuf {
    match dirs::config_dir() {
        Some(path) => path.join("muxmate"),
        None => {
            eprintln!("Could  not find a config directory");
            std::process::exit(1);
        }
    }
}
