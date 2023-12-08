use std::collections::{HashMap, HashSet};

pub fn day4_p1() {
    let input_str = include_str!("../examples/day4_input.txt");
    let mut winning_numbers: Vec<Vec<u32>> = Vec::new();
    for card in input_str.lines() {
        let win_set: HashSet<u32>;
        let our_set: HashSet<u32>;
        let outputs: Vec<&str> = card.split(": ").last().unwrap().split(" | ").collect();
        match outputs.as_slice() {
            [win, ours] => {
                win_set = HashSet::from_iter(
                    win.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()),
                );
                our_set = HashSet::from_iter(
                    ours.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()),
                );
            }
            _ => panic!("unexpected split length"),
        }
        winning_numbers.push(
            win_set
                .intersection(&our_set)
                .cloned()
                .collect::<Vec<u32>>(),
        );
    }
    let factorial_sum = winning_numbers
        .iter()
        .map(|x| {
            if x.is_empty() {
                0
            } else {
                2u32.pow(x.len() as u32 - 1)
            }
        })
        .sum::<u32>();
    println!("{factorial_sum:#?}");
}

pub fn day4_p2() {
    let input_str = include_str!("../examples/day4_input.txt");
    // collect sets of winning values same as before
    let mut winning_numbers: Vec<Vec<u32>> = Vec::new();
    for card in input_str.lines() {
        let win_set: HashSet<u32>;
        let our_set: HashSet<u32>;
        let outputs: Vec<&str> = card.split(": ").last().unwrap().split(" | ").collect();
        match outputs.as_slice() {
            [win, ours] => {
                win_set = HashSet::from_iter(
                    win.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()),
                );
                our_set = HashSet::from_iter(
                    ours.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap()),
                );
            }
            _ => panic!("unexpected split length"),
        }
        winning_numbers.push(
            win_set
                .intersection(&our_set)
                .cloned()
                .collect::<Vec<u32>>(),
        );
    }
    // create hashmap of how many wins per game
    let win_hashmap = winning_numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1, x.len() as u32))
        .collect::<HashMap<u32, u32>>();

    // create hashmap of how many of each card we have, starts from 1
    let mut cards_hashmap = (1..=winning_numbers.len())
        .map(|x| (x as u32, 1))
        .collect::<HashMap<u32, u32>>();

    // start from card 1 and iteratively add the future cards
    for i in 1..=winning_numbers.len() as u32 {
        // find how many wins we had
        let wins = win_hashmap.get(&i).unwrap().clone();
        // find how many of the current card we had to multiply effect
        let num_card = cards_hashmap.get(&i).unwrap().clone();

        if wins == 0 {
            continue;
        }

        // for each increment add N of the subsequent cards
        for win_increment in 1..=wins {
            let val = cards_hashmap.get_mut(&(i + win_increment)).unwrap();
            *val += num_card;
        }
    }

    println!("Total cards: {}", cards_hashmap.values().sum::<u32>());
}
