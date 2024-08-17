use crate::init;

pub fn init_handler() {
    match init::init_config() {
        Ok(path) => println!("Initialized config in {}", path.display()),
        Err(er) => eprintln!("Could not create files/directories: {}", er),
    }
}
