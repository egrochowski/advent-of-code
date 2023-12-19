use aoc::read_file;

fn main() {
    let oasis = read_file("09");
    println!("{}", solve(&oasis.clone(), false));
    println!("{}", solve(&oasis.clone(), true));
}

fn solve(oasis: &str, is_part_one: bool) -> i32 {
    let mut result = 0;

    for line in oasis.lines() {

        let mut derived: Vec<i32> = parse_line(line);
        if is_part_one { derived.reverse(); }
        let mut stack = vec![derived[derived.len()-1]];

        while derived.iter().sum::<i32>() != 0 {
            let mut buffer = Vec::new();
            for i in 1..derived.len() {
                buffer.push(derived[i] - derived[i-1]);
            }
            stack.push(buffer[buffer.len()-1]);
            derived = buffer;
        }

        while stack.len() > 1 {
            let derived = stack.pop().unwrap();
            let n = stack.len()-1;
            stack[n] += derived;
        }

        result += stack.pop().unwrap();
    }
    result
}

fn parse_line(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}
