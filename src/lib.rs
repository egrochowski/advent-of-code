use std::{fs, env};

pub fn read_file(file: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("inputs").join(file); 
    return fs::read_to_string(path).unwrap();
}

pub fn parse_to_matrix(file: &str) -> Vec<Vec<char>> {
    return read_file(file)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}   

