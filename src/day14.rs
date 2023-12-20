use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
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

    let mut total_load = 0;
    for col in 0..input[0].len() {
        let mut col_vector: Vec<Stone> = input.iter().map(|x| x[col].clone()).collect();
        col_vector = roll_column(col_vector);
        total_load += column_load(&col_vector);
    }

    println!("Total load: {:#?}", total_load);
}
