use aoc::parse_input;
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn main() { 
    let scratchcard = parse_input("04");
    println!("{}", part_one(&scratchcard.clone()));
    println!("{}", part_two(&scratchcard.clone()));
}

fn part_one(scratchcard: &str) -> i32 {
    let re = Regex::new(r"(\w+)").unwrap();
    let mut total_winnings = 0;
    for card in scratchcard.lines() {
        let mut num_win: i32 = 0;
        let mut card = card[card.find(":").unwrap()+1..].split("|");
        let winning_nums = get_set(card.next().unwrap()); 
        let owned_nums = card.next().unwrap();
        for (_, [num]) in  re.captures_iter(owned_nums).map(|num| num.extract()) {
            if winning_nums.contains(num) {
                num_win += 1;
            }
        }
        if num_win > 0 { total_winnings += 2i32.pow((num_win-1).try_into().unwrap_or_default()) }
    }
    return total_winnings;
}

fn part_two(scratchcard: &str) -> i32 {
    let re = Regex::new(r"(\w+)").unwrap();
    let mut total_cards = 0;

    let mut map: HashMap<usize, i32> = HashMap::new(); // Card num: Copies

    for (idx, card) in scratchcard.lines().enumerate() {
        let mut card = card[card.find(":").unwrap()+1..].split("|");
        let winning_nums = get_set(card.next().unwrap()); 
        let owned_nums = card.next().unwrap();
        let card_num = idx + 1;

        if !map.contains_key(&(card_num)) {
            map.insert(card_num, 1);
        } else {
            map.insert(card_num, *map.get(&(card_num)).unwrap() + 1);
        }

        while *map.get(&(card_num)).unwrap() > 0 { // has copies 
            total_cards += 1;
            map.insert(card_num, *map.get(&(card_num)).unwrap() - 1);
            let mut wins = 0;
            for (_, [num]) in  re.captures_iter(owned_nums).map(|num| num.extract()) {
                if winning_nums.contains(num) {
                    wins += 1;
                }
            }

            // add wins to cards
            for i in card_num+1..wins+card_num+1 {
                if !map.contains_key(&i) {
                    map.insert(i, 0);
                }
                map.insert(i, *map.get(&i).unwrap() + 1);
            }
        }
    }
    return total_cards as i32;
}

fn get_set(card: &str) -> HashSet<&str> {
    let re = Regex::new(r"(\w+)").unwrap();
    let mut set = HashSet::new();
    for (_, [num]) in re.captures_iter(card).map(|num| num.extract()) {
        set.insert(num);
    };
    return set;
}
