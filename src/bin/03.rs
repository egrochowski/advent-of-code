use aoc::parse_to_matrix;

fn main () {
    let input = parse_to_matrix("03");
    println!("{}", part_one(&mut input.clone()));
    println!("{}", part_two(&mut input.clone()));
}

fn part_one(input: &mut Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..input.len() {
        let mut j = 0;
        while j < input[i].len() {
            let num = get_num(input, i, j);
            let num_size = num.len();
            let num = num.parse::<i32>().unwrap_or(0);
            j += num_size;
            if num > 0 && is_valid_num(input.clone(), i, j, num_size) {
                sum += num;
            }
            j += 1;
        }
    }
    sum
}

fn part_two(input: &mut Vec<Vec<char>>) -> i32 {

    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '*' {
                sum += get_gear_ratio(input, i, j);
            }
        }
    }
    sum
}

fn is_valid_num(grid: Vec<Vec<char>>, row: usize, col: usize, num_size: usize) -> bool {
    let m_range = if row > 0 { row-1..row+2 } else { 0..2 };
    let n_range = if col-num_size > 0 { col-num_size-1..col+1 } else { 0..col+1 };
    for m in m_range.clone() {
        for n in n_range.clone() { 
            if m < grid.len() && n < grid[m].len() && is_special_char(grid[m][n]) {
                return true;
            }
        }
    }
    return false;
}

fn is_special_char(ch: char) -> bool {
    ch != '.' && !ch.is_numeric() 
}


fn get_gear_ratio(grid: &mut Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut gear_ratio = 1;
    let mut adj_nums = 0;
        let m_range = if row > 0 { row-1..row+2 } else { 0..2 };
        let n_range = if col > 0 { col-1..col+2 } else { 0..col+2 };
        for m in m_range.clone() {
            for n in n_range.clone() {
                if grid[m][n].is_numeric() {
                    let num = get_num(grid, m, n).parse::<i32>().unwrap_or(1);
                    gear_ratio *= num;
                    adj_nums += 1; 
                }
            }
        }
    if adj_nums == 2 { gear_ratio } else { 0 }
}

fn get_num(grid: &mut Vec<Vec<char>>, row: usize, mut start: usize) -> String {
    let mut num = String::new();
   
    // Find beginning of num
    while start > 0 && grid[row][start-1].is_numeric() {
        start -= 1;
    }
    while start < grid[row].len () && grid[row][start].is_numeric() {
        num.push(grid[row][start]);
        grid[row][start] = '.'; // Remove number from board
        start += 1;
    }
    num
}
