use std::collections::HashSet;

pub fn day4_p1() {
    let input_str = include_str!("../examples/day4_input.txt");
    let mut winning_numbers: Vec<Vec<u32>> = Vec::new();
    for card in input_str.lines() {
        let mut win_set: HashSet<u32> = HashSet::new();
        let mut our_set: HashSet<u32> = HashSet::new();
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
}
