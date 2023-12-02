use aoc::parse_input;

fn main () {
    let input = parse_input("01");
    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}

fn part_one(input: String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut num  = String::new();
        let mut digs = line.chars().filter(|c| c.is_numeric());

        let left = digs.next();
        let right = digs.last();

        if left.is_some() && right.is_some() { 
            num.push(left.unwrap());
            num.push(right.unwrap());
        } else {
            num.push(left.unwrap_or('0'));
            num.push(left.unwrap_or('0'));
        }

        sum += num.parse::<i32>().unwrap_or_default();
    }
    sum
}

fn part_two(input: String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let new_line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        sum += part_one(new_line);
    }
    sum 
}
