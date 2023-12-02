use std::{fs, env};

pub fn parse_input(file: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("inputs").join(file); 
    fs::read_to_string(path).unwrap()
}
