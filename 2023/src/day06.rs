fn get_day6_input_p1() -> Vec<(u64, u64)> {
    let mut input_lines = include_str!("../examples/day6_input.txt").lines();

    let time_dists: Vec<(u64, u64)> = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .flat_map(|x| {
            x.trim()
                .split_ascii_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
        })
        .zip(
            input_lines
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .flat_map(|x| {
                    x.trim()
                        .split_ascii_whitespace()
                        .map(|i| i.parse::<u64>().unwrap())
                }),
        )
        .collect();
    time_dists
}

fn get_day6_input_p2() -> (u64, u64) {
    let mut input_lines = include_str!("../examples/day6_input.txt").lines();

    let time: u64 = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let dist: u64 = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    (time, dist)
}

/// Get distance based on pressing and total time
/// each unit of pressed time increases speed by 1, which is then travelled at for remainder
fn get_distance(total_time: u64, time_pressed: u64) -> u64 {
    let time_not_pressed = total_time - time_pressed;
    time_pressed * time_not_pressed
}

pub fn day6_p1() {
    let time_dists = get_day6_input_p1();
    let winning_ways: Vec<usize> = time_dists
        .into_iter()
        .map(|(time, dist)| {
            (0..=time)
                .map(|x| get_distance(time, x))
                .filter(|x| x > &dist)
                .count()
        })
        .collect();
    println!(
        "Product of ways to win: {:#?}",
        winning_ways.iter().product::<usize>()
    );
}

pub fn day6_p2() {
    let (time, dist) = get_day6_input_p2();
    let winning_ways = (0..=time)
        .map(|x| get_distance(time, x))
        .filter(|x| x > &dist)
        .count();
    println!("Ways to win: {:#?}", winning_ways);
}
