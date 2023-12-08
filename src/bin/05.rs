use aoc::parse_input;
use core::ops::Range;
use std::collections::HashSet;


fn main () {
    let almanac = parse_input("05");
    println!("{}", part_one(&almanac.clone()));
    println!("{}", part_two(&almanac.clone()));

}

fn part_one(almanac: &str) -> u64 {
    let mut almanac = almanac.split("\n\n");
    let seeds = parse_seed_header(almanac.next().unwrap()); // parse header
    let mappings = get_mappings(almanac.collect());
    let mut min_seed = u64::MAX;
    for seed in seeds.clone() {
        let mut current_seed = seed;
        for maps in mappings.clone() {
            for ranges in maps.clone() {
                let (dest, src) = ranges;
                if src.contains(&current_seed) {
                    current_seed = current_seed - src.start + dest.start;
                    break;
                }
            }
        }
        min_seed = min_seed.min(current_seed);
    }
    min_seed
}

fn part_two(almanac: &str) -> u64 {
    let mut almanac = almanac.split("\n\n");
    let seeds = parse_seed_header_to_range(almanac.next().unwrap()); // parse header
    let mappings = get_mappings(almanac.collect());
    let mut min_seed = u64::MAX;
    for seed_range in seeds.clone() {
        for seed in seed_range {
            let mut current_seed = seed;
            for maps in mappings.clone() {
                for ranges in maps.clone() {
                    let (dest, src) = ranges;
                    if src.contains(&current_seed) {
                        current_seed = current_seed - src.start + dest.start;
                        break;
                    }
                }
            }
            min_seed = min_seed.min(current_seed);
        }
    }
    min_seed
}

fn get_mappings(almanac: Vec<&str>) -> Vec<Vec<(Range<u64>, Range<u64>)>> {
    let mut mappings: Vec<Vec<(Range<u64>, Range<u64>)>> = Vec::new();
    for maps in almanac {
        mappings.push(get_ranges(maps));
    }
    mappings
}

fn get_ranges(maps: &str) -> Vec<(Range<u64>, Range<u64>)> {
    let mut ranges: Vec<(Range<u64>, Range<u64>)> = Vec::new();
    for map in maps.trim().split("\n").skip(1) { // skip mapping header
        ranges.push(parse_ranges(map));
    }
    ranges
}

fn parse_seed_header(header: &str) -> Vec<u64> {
    let mut seeds = Vec::new();
    let header = header.split_once(":").unwrap();
    let (_, nums) = header;
    nums.split_whitespace().for_each(|num| {
        seeds.push(num.parse().unwrap());
    });
    seeds
}

fn parse_seed_header_to_range(header: &str) -> Vec<Range<u64>> {
    let mut seeds = Vec::new();
    let header = header.split_once(":").unwrap();
    let (_, nums) = header;
    let mut i = 0;
    let nums: Vec<&str>  = nums.split_whitespace().collect();
    while i < nums.len() {
        let seed = nums[i].parse().unwrap();
        let range = seed..(seed+nums[i+1].parse::<u64>().unwrap());
        seeds.push(range);
        i += 2;
    };
    seeds
}

fn parse_ranges(mapping: &str) -> (Range<u64>, Range<u64>) {
    let mapping: Vec<&str> =  mapping.split_whitespace().collect();
    let dest = mapping[0].parse::<u64>().unwrap();   
    let src  = mapping[1].parse::<u64>().unwrap(); 
    let len  = mapping[2].parse::<u64>().unwrap(); 
    (dest..dest+len, src..src+len)
}

fn has(mut left: u64, mut right: u64, n: u64) -> bool {
    while left < right {
        let mid = left + right / 2; 
        if mid == n { 
            return true; 
        } else if mid < left {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    false
}

#[cfg(test)]
mod t_tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn check() {
        let almanac_test = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(35, part_one(almanac_test));
        assert_eq!(46, part_two(almanac_test));
    }
}
