use aoc::parse_input;

fn main() {
    let paper = parse_input("06");
    println!("{}", part_one(&paper.clone()));
    println!("{}", part_two(&paper.clone()));
}

fn part_one(paper: &str) -> u64 {
    
    let paper = parse_paper(paper);
    let mut n_ways = 1;
    let times = &paper[0];
    let dists = &paper[1];
    for i in 0..times.len() {
        // calculate wins 
        n_ways *= find_num_possible_wins(times[i], dists[i]); 
    }
    if n_ways > 1 { n_ways } else { 0 } 
}

fn part_two(paper: &str) -> u64 {
    let paper = concat_fields(paper);
    let time = paper[0];
    let dist = paper[1];
    find_num_possible_wins(time, dist)
}

fn find_num_possible_wins(time: u64, dist: u64) -> u64 {
    let last_win = find_last_win(time / 2, time, dist);
    let first_win = time - last_win;
    let wins = last_win - first_win;
    wins - 1
}

fn parse_paper(paper: &str) -> Vec<Vec<u64>> {
    let mut parsed: Vec<Vec<u64>> = Vec::new();
    for line in paper.lines() {
        let result: Vec<u64> = line.split_whitespace()
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        parsed.push(result);
    }
    parsed
}

fn concat_fields(paper: &str) -> Vec<u64> {
    let mut parsed: Vec<u64> = Vec::new();
    for line in paper.lines() {
        let result: Vec<&str> = line
            .split_whitespace()
            .skip(1)
            .collect();
        parsed.push(result.join("").parse().unwrap());
    }
    parsed
}

fn find_last_win(mut start: u64, mut end: u64, target: u64) -> u64 {
    let n = end;
    while start < end {
        let time = (start + end) / 2;
        let dist = time * (n - time);
        if dist <= target {
            end = time;
        } else {
            start = time + 1;
        }
    }
    start
}
