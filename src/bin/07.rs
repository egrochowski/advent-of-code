use aoc::read_file;
use std::collections::HashMap;
use counter::Counter;

#[derive(Debug)]
enum Strengths {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
    None,
}

fn main () {
    let transactions = read_file("07");
    println!("{}", part_one(&transactions.clone()));
    println!("{}", part_two(&transactions.clone()));
}

fn part_one(transactions: &str) -> u32 {
    let transactions = sort(parse(&transactions));
    let mut result = 0;
    for (i, (_, wager)) in transactions.iter().enumerate() {
        result += (*wager) * (i + 1) as u32;
    }
    result    
}

fn part_two(transactions: &str) -> u32 {
    let transactions = sort_by_wildcard(parse(&transactions));
    let mut result = 0;
    for (i, (_, wager)) in transactions.iter().enumerate() {
        result += (*wager) * (i + 1) as u32;
    }
    result    
}

fn parse(games: &str) -> Vec<(&str, u32)> {
    let mut parsed:  Vec<(&str, u32)> = Vec::new();
    for line in games.lines() {
        let result = line
            .split_once(" ")
            .map(|(hand, wager)| (hand, wager.parse::<u32>().unwrap()))
            .unwrap();
        parsed.push(result);
    }
    parsed
}

fn sort(transactions: Vec<(&str, u32)>) ->  Vec<(&str, u32)>   {
    let mut ranks: Vec<Vec<(&str, u32)>> = vec![vec![]; 7];
    for hand in transactions {
        match determine_rank(hand.0) {
            Strengths::HighCard => ranks[0].push(hand),
            Strengths::OnePair => ranks[1].push(hand),
            Strengths::TwoPair => ranks[2].push(hand),
            Strengths::ThreeKind => ranks[3].push(hand),
            Strengths::FullHouse => ranks[4].push(hand),
            Strengths::FourKind => ranks[5].push(hand),
            Strengths::FiveKind => ranks[6].push(hand),
            _ => ()
        }
    }

    ranks
        .iter()
        .flat_map(|rank| {
            sort_row(rank.clone())
        })
        .collect()
}

fn sort_row(row: Vec<(&str, u32)>) -> Vec<(&str, u32)> {
    let map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    let mut clone = row.clone();
    clone.sort_by(|a,b| {
        let (a_hand, _) = a;
        let (b_hand, _) = b;

        let a_hand = a_hand.chars().collect::<Vec<char>>();
        let b_hand = b_hand.chars().collect::<Vec<char>>();
        
        let mut i = 0;
        while i < a_hand.len() {
            if a_hand[i] != b_hand[i] {
                return map.get(&a_hand[i]).unwrap().cmp(&map.get(&b_hand[i]).unwrap());
            }
            i += 1;
        }
        return map.get(&a_hand[0]).unwrap().cmp(&map.get(&b_hand[0]).unwrap());
    });
    clone
}

fn sort_by_wildcard(transactions: Vec<(&str, u32)>) -> Vec<(&str, u32)> {
    let mut ranks: Vec<Vec<(&str, u32)>> = vec![vec![];7];
    for hand in transactions {
        match determine_new_rank(hand.0) {
            Strengths::HighCard => ranks[0].push(hand),
            Strengths::OnePair => ranks[1].push(hand),
            Strengths::TwoPair => ranks[2].push(hand),
            Strengths::ThreeKind => ranks[3].push(hand),
            Strengths::FullHouse => ranks[4].push(hand),
            Strengths::FourKind => ranks[5].push(hand),
            Strengths::FiveKind => ranks[6].push(hand),
            _ => ()
        }
    }

    ranks
        .iter()
        .flat_map(|rank| {
            sort_row_by_wildcard(rank.clone())
        })
        .collect()
}

fn sort_row_by_wildcard(row: Vec<(&str, u32)>) -> Vec<(&str, u32)> {
    let map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
    let mut clone = row.clone();
    clone.sort_by(|a,b| {
        let (a_hand, _) = a;
        let (b_hand, _) = b;

        let a_hand = a_hand.chars().collect::<Vec<char>>();
        let b_hand = b_hand.chars().collect::<Vec<char>>();
        
        let mut i = 0;
        while i < a_hand.len() {
            if a_hand[i] != b_hand[i] {
                return map.get(&a_hand[i]).unwrap().cmp(&map.get(&b_hand[i]).unwrap());
            }
            i += 1;
        }
        return map.get(&a_hand[0]).unwrap().cmp(&map.get(&b_hand[0]).unwrap());
    });
    clone
}

fn determine_new_rank(hand: &str) -> Strengths {
    let hand = hand.replace("J", &find_most_freq_card(&hand).to_string());
    determine_rank(&hand)
}

fn determine_rank(hand: &str) -> Strengths {
    let counts = get_char_freqs(hand);
    let max_num = *counts.values().max().unwrap_or(&0);
    match counts.len() {
        1 => Strengths::FiveKind,
        2 => if max_num == 4 { Strengths::FourKind } else { Strengths::FullHouse },
        3 => if max_num == 3 { Strengths::ThreeKind } else { Strengths::TwoPair },
        4 => Strengths::OnePair, 
        5 => Strengths::HighCard,
        _ => Strengths::None
    }
}

fn find_most_freq_card(hand: &str) -> char {
    let mut counts = get_char_freqs(hand);
    counts.remove(&'J'); // don't count J
    *counts
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap_or(&'J')
}

fn get_char_freqs(hand: &str) -> Counter<char, u32> {
    hand.chars().collect::<Counter<char, u32>>()
}
