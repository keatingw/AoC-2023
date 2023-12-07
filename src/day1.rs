use std::collections::HashMap;

pub fn day_1_p1() {
    let input_strings: Vec<&str> = include_str!("../examples/day1_input.txt").lines().collect();
    let digits: Vec<u32> = input_strings
        .iter()
        .map(|l| l.chars().filter(|x| x.is_digit(10)).collect::<Vec<char>>())
        .map(|x| {
            (x.first().unwrap().to_string() + &x.last().unwrap().to_string())
                .parse::<u32>()
                .unwrap()
        })
        .collect();
    println!("digit list: {:#?}", digits);
    println!("digits sum: {:#?}", digits.iter().sum::<u32>());
}

pub fn day_1_p2() {
    let numeric_map: HashMap<&str, &str> = HashMap::from_iter(vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let input_strings: Vec<&str> = include_str!("../examples/day1_input.txt").lines().collect();
    let mut output_numbers: Vec<u32> = Vec::new();

    for &line in &input_strings {
        let mut leftmost: String = "".to_string();
        let mut rightmost: String = "".to_string();
        for i in 0..line.len() {
            if leftmost != "" {
                break;
            }
            let cur_char = line.chars().collect::<Vec<char>>()[i];
            if cur_char.is_digit(10) {
                leftmost = cur_char.to_string();
            }
            for &numtext in numeric_map.keys() {
                if line[i..].starts_with(numtext) {
                    leftmost = numeric_map.get(numtext).unwrap().to_string();
                }
            }
        }

        for i in 1..=line.len() {
            if rightmost != "" {
                break;
            }
            let cur_char = line.chars().collect::<Vec<char>>()[line.len() - i];
            if cur_char.is_digit(10) {
                rightmost = cur_char.to_string();
            }
            for &numtext in numeric_map.keys() {
                if line[..=line.len() - i].ends_with(numtext) {
                    rightmost = numeric_map.get(numtext).unwrap().to_string();
                }
            }
        }
        output_numbers.push((leftmost + &rightmost).parse::<u32>().unwrap());
    }
    println!("{:#?}", output_numbers);
    println!("{}", output_numbers.iter().sum::<u32>());
}
