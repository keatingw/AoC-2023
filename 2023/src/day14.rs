use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Stone {
    Round,
    Cube,
    Space,
}

impl From<char> for Stone {
    fn from(value: char) -> Self {
        match value {
            'O' => Stone::Round,
            '#' => Stone::Cube,
            '.' => Stone::Space,
            _ => panic!("Can't make Stone type from char {value}"),
        }
    }
}

fn get_day14_input(path: &str) -> Vec<Vec<Stone>> {
    let input_str = fs::read_to_string(path).unwrap();
    input_str
        .lines()
        .map(|l| l.chars().map(Stone::from).collect())
        .collect()
}

// Recursive rolling function to operate column-wise
fn roll_column(mut column: Vec<Stone>) -> Vec<Stone> {
    let mut num_changes = 0;

    // pass down over the length of the column
    for i in 1..column.len() {
        // only operate on round stones
        if column[i] == Stone::Round {
            let mut new_index = i.clone();
            // go backwards up the column finding positions
            for newpos in (0..i).rev() {
                // if it's a gap then add this as the latest candidate
                if column[newpos] == Stone::Space {
                    new_index = newpos;
                // if full then stop
                } else {
                    break;
                }
            }
            // move stone if we found a higher up position
            if new_index != i {
                num_changes += 1;
                column[new_index] = Stone::Round;
                column[i] = Stone::Space;
            }
        }
    }
    // convergence check: if we didn't make any changes then stop
    if num_changes != 0 {
        roll_column(column)
    } else {
        column
    }
}

fn column_load(column: &Vec<Stone>) -> usize {
    let mut total_load = 0;
    for (idx, i) in column.iter().enumerate() {
        match i {
            Stone::Round => total_load += column.len() - idx,
            _ => (),
        }
    }
    total_load
}

pub fn day14_p1() {
    let input = get_day14_input("examples/day14_input.txt");

    let mut cache = HashMap::new();
    let mut total_load = 0;

    for col in 0..input[0].len() {
        let mut col_vector: Vec<Stone> = input.iter().map(|x| x[col].clone()).collect();
        col_vector = roll_column_memo(col_vector, &mut cache);
        total_load += column_load(&col_vector);
    }

    println!("Total load: {:#?}", total_load);
}

// memoised version
fn roll_column_memo(column: Vec<Stone>, cache: &mut HashMap<Vec<Stone>, Vec<Stone>>) -> Vec<Stone> {
    match cache.get(&column) {
        Some(x) => x.to_vec(),
        None => {
            let out = roll_column(column.clone());
            cache.insert(column, out.clone());
            out
        }
    }
}

// Recursive rolling function to operate column-wise
fn roll_box(mut square: Vec<Vec<Stone>>, direction: Direction) -> Vec<Vec<Stone>> {
    match direction {
        Direction::North => {
            for col in 0..square[0].len() {
                let mut column: Vec<Stone> = square.iter().map(|x| x[col].clone()).collect();
                column = roll_column(column);
                for (idx, i) in column.into_iter().enumerate() {
                    square[idx][col] = i;
                }
            }
        }
        Direction::South => {
            for col in 0..square[0].len() {
                let mut column: Vec<Stone> = square.iter().map(|x| x[col].clone()).collect();
                // reverse before and after to get south for 'free' using roll column
                column.reverse();
                column = roll_column(column);
                column.reverse();
                for (idx, i) in column.into_iter().enumerate() {
                    square[idx][col] = i;
                }
            }
        }
        Direction::West => {
            for row in 0..square.len() {
                let mut row_vec: Vec<Stone> = square[row].clone();
                row_vec = roll_column(row_vec);
                square[row] = row_vec;
            }
        }
        Direction::East => {
            for row in 0..square.len() {
                let mut row_vec: Vec<Stone> = square[row].clone();
                // reverse before and after to get east for 'free' using roll column
                row_vec.reverse();
                row_vec = roll_column(row_vec);
                row_vec.reverse();
                square[row] = row_vec;
            }
        }
    }
    square
}

pub fn day14_p2() {
    let mut input = get_day14_input("examples/day14_input.txt");

    let mut seen_set: HashSet<Vec<Vec<Stone>>> = HashSet::new();
    let mut seen_vec: Vec<Vec<Vec<Stone>>> = vec![];
    seen_set.insert(input.clone());
    seen_vec.push(input.clone());
    let mut print;
    let mut iter = 0;
    loop {
        if iter % 1 == 0 {
            print = true;
        } else {
            print = false;
        }
        if print {
            println!("Running {iter:>10}");
        }

        // do cycle
        input = roll_box(input, Direction::North);
        input = roll_box(input, Direction::West);
        input = roll_box(input, Direction::South);
        input = roll_box(input, Direction::East);

        // add to seen cycles or break if already seen
        if !seen_set.insert(input.clone()) {
            break;
        }
        // add to vec to store index positions
        seen_vec.push(input.clone());
        iter += 1;
    }

    let cycles = 1_000_000_000;
    // save the first time we saw the end point of the cycle
    let first_cycle_idx = seen_vec.iter().position(|x| x == &input).unwrap();
    println!("First pos: {}", first_cycle_idx);
    // end position is number of steps into the cycle, plus start point of cycle
    let final_grid = seen_vec
        [(cycles - first_cycle_idx) % (seen_vec.len() - first_cycle_idx) + first_cycle_idx]
        .clone();
    let mut total_load = 0;
    for col in 0..input[0].len() {
        let col_vector: Vec<Stone> = final_grid.iter().map(|x| x[col].clone()).collect();
        total_load += column_load(&col_vector);
    }

    println!("Total load: {:#?}", total_load);
}
