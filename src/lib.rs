use std::{fs, env};

pub fn parse_input(file: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("inputs").join(file); 
    fs::read_to_string(path).unwrap()
}

pub fn parse_to_matrix(file: &str) -> Vec<Vec<char>> {
    parse_input(file)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}   
