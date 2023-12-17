use aoc::read_file;
use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

fn main() {
    let map = read_file("08");
    println!("{}", part_one(&map.clone()));
    println!("{}", part_two(&map.clone()));
}

fn part_one(map: &str) -> u32 { 
    let mut lines = map.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next(); // skip white space
    let map = parse_map(lines.clone());
    let mut key = "AAA";
    let mut count = 0;
    while key != "ZZZ" {
        let instruction = instructions[count % instructions.len()];
        let (l, r) = *map.get(&key).unwrap();
        if instruction == 'L' {
            key = l;
        } else {
            key = r;
        }
        count += 1;
    }
    count as u32
}

fn part_two(map: &str) -> u64 {
    let mut lines = map.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next(); // skip white space
    let map = parse_map(lines.clone());
    let mut a_keys = find_keys_ending_in_a(map.clone());
    let mut count = 0;
    let mut steps = Vec::new();
    while !a_keys.is_empty() {
        let mut key = a_keys.pop().unwrap();
        while !is_end(key) {
            let instruction = instructions[count % instructions.len()];
            let (l, r) = *map.get(&key).unwrap();
            if instruction == 'L' {
                key = l;
            } else {
                key = r;
            }
            count += 1;
        }
        steps.push(count as u64);
        count = 0;
    }
    let mut result = lcm(steps[0], steps[1]);
    for i in 2..steps.len() {
        result = lcm(result, steps[i]);
    }
    result
}

fn find_keys_ending_in_a<'a> (map: HashMap<&'a str, (&'a str, &'a str)>) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();
    let re = Regex::new(r"(..A)").unwrap();
    for key in map.keys() {
       if re.is_match(key) {
            matches.push(key);
       }
    }
    matches
}

fn is_end(key: &str) -> bool {
    let re = Regex::new(r"(..Z)").unwrap();
    re.is_match(key)
}

fn parse_instructions(instructions: &str) -> Vec<char> {
    instructions.chars().collect()
}

fn parse_map(lines: std::str::Lines) -> HashMap<&str, (&str, &str)> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new(); 
    lines
        .for_each(|m| {
            let (key, dests) = m.split_once("=").unwrap();
            let (a,b) =  dests.trim().split_once(",").unwrap();
            let a = a.strip_prefix("(").unwrap().trim();
            let b = b.strip_suffix(")").unwrap().trim();
            map.insert(key.trim(), (a,b)); 
        });
    map
}
