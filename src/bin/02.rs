use aoc::parse_input;
use regex::Regex;

fn main() {
    let input = parse_input("02");
    println!("{}", part_one(&input.clone()));
    println!("{}", part_two(&input.clone()));
}

fn part_one(input: &str) -> i32 {
    let mut valid_games = 0;
    for line in input.lines() {
        let game_id = get_id(line);
        let line = &line[line.find(":").unwrap()..];
        if is_valid_game(line) {
            valid_games += game_id;
        }
    }
    valid_games
}

fn part_two(input: &str) -> i32 {
    let mut min_cubes = 0;
    for line in input.lines() {
        let line = &line[line.find(":").unwrap()..];
        min_cubes += get_min_cubes(line);
    }
    min_cubes
}

fn is_valid_game(line: &str) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
    let mut is_valid = true;
    line.split(";").for_each(|game| {
        re.captures_iter(game).for_each(|cap| {
            let cubes = cap.get(1).unwrap().as_str().trim();
            let color = cap.get(2).unwrap().as_str().trim();

            match color { 
                "red" => is_valid &= parse_num(cubes) <= MAX_RED,
                "blue" => is_valid &= parse_num(cubes) <= MAX_BLUE,
                "green" => is_valid &= parse_num(cubes) <= MAX_GREEN,
                _ => ()
            };
        });
    });
    is_valid
}

fn get_min_cubes(line: &str) -> i32 {

    let re = Regex::new(r"(\w+)\s+(\w+)").unwrap();
    let mut max_red = 0; 
    let mut max_green = 0; 
    let mut max_blue = 0; 
    line.split(";").for_each(|game| {
        re.captures_iter(game).for_each(|cap| {
            let cubes = parse_num(cap.get(1).unwrap().as_str().trim());
            let color = cap.get(2).unwrap().as_str().trim();

            match color { 
                "red" => max_red = max_red.max(cubes),
                "blue" => max_blue = max_blue.max(cubes),
                "green" => max_green= max_green.max(cubes),
                _ => ()
            };
        });
    });
    max_red * max_green * max_blue
}

fn parse_num(num: &str) -> i32 {
    num.trim().parse::<i32>().unwrap()
}

fn get_id(line: &str) -> i32 {
    let re = Regex::new(r"(\w+):").unwrap();
    let id = re.find(line).unwrap().as_str();
    parse_num(&id[..id.len()-1])
}

