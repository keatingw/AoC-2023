use std::fs;

fn get_day13_input(path: &str) -> Vec<Vec<Vec<char>>> {
    let input_str = fs::read_to_string(path).unwrap();
    input_str
        .split("\n\n")
        .map(|x| x.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn check_reflection_vertical(index: usize, square: &[Vec<char>]) -> bool {
    (0..index)
        .rev()
        .zip(index..square.len())
        .all(|(l, r)| square[l] == square[r])
}

fn check_reflection_horizontal(index: usize, square: &[Vec<char>]) -> bool {
    (0..index).rev().zip(index..square[0].len()).all(|(l, r)| {
        square.iter().map(|x| x[l]).collect::<Vec<char>>()
            == square.iter().map(|x| x[r]).collect::<Vec<char>>()
    })
}

pub fn day13_p1() {
    let input = get_day13_input("examples/day13_input.txt");
    let mut running_sum = 0;
    for i in input {
        for idx in 1..i[0].len() {
            if check_reflection_horizontal(idx, &i) {
                running_sum += idx;
                break;
            }
        }
        for idx in 1..i.len() {
            if check_reflection_vertical(idx, &i) {
                running_sum += idx * 100;
                break;
            }
        }
    }
    println!("Total sum: {running_sum}");
}
