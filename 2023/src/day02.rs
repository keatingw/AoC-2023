#[derive(Debug)]
pub struct ColourCounts {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub fn day_2_p1() {
    let input_strings: Vec<&str> = include_str!("../examples/day2_input.txt").lines().collect();
    let mut possible_games: Vec<usize> = Vec::new();

    for (idx, game) in input_strings.iter().enumerate() {
        let mut colour_count = ColourCounts {
            red: 0,
            green: 0,
            blue: 0,
        };

        let rounds: Vec<&str> = game.split(": ").last().unwrap().split("; ").collect();

        for round in rounds {
            for count in round.split(", ") {
                let per_c_counts = count.split(" ").collect::<Vec<&str>>();
                match per_c_counts[..] {
                    [x, "red"] => {
                        colour_count.red = colour_count.red.max(x.parse::<u32>().unwrap())
                    }
                    [x, "green"] => {
                        colour_count.green = colour_count.green.max(x.parse::<u32>().unwrap())
                    }
                    [x, "blue"] => {
                        colour_count.blue = colour_count.blue.max(x.parse::<u32>().unwrap())
                    }
                    _ => panic!(),
                }
            }
        }
        if colour_count.red <= 12 && colour_count.green <= 13 && colour_count.blue <= 14 {
            possible_games.push(idx + 1)
        }
    }
    println!(
        "sum of possible={:#?}",
        possible_games.iter().sum::<usize>()
    );
}

pub fn day_2_p2() {
    let input_strings: Vec<&str> = include_str!("../examples/day2_input.txt").lines().collect();
    let mut game_powers: Vec<u32> = Vec::new();

    for game in input_strings {
        let mut colour_count = ColourCounts {
            red: 0,
            green: 0,
            blue: 0,
        };

        let rounds: Vec<&str> = game.split(": ").last().unwrap().split("; ").collect();

        for round in rounds {
            for count in round.split(", ") {
                let per_c_counts = count.split(" ").collect::<Vec<&str>>();
                match per_c_counts[..] {
                    [x, "red"] => {
                        colour_count.red = colour_count.red.max(x.parse::<u32>().unwrap())
                    }
                    [x, "green"] => {
                        colour_count.green = colour_count.green.max(x.parse::<u32>().unwrap())
                    }
                    [x, "blue"] => {
                        colour_count.blue = colour_count.blue.max(x.parse::<u32>().unwrap())
                    }
                    _ => panic!(),
                }
            }
        }
        game_powers.push(colour_count.red * colour_count.green * colour_count.blue)
    }
    println!("sum of powers={:#?}", game_powers.iter().sum::<u32>());
}
