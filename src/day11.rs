use std::fs;

fn get_day11_input(path: &str) -> Vec<Vec<bool>> {
    let input_str = fs::read_to_string(path).unwrap();
    input_str
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

pub fn day11_p1() {
    let mut galaxy_grid = get_day11_input("examples/day11_input.txt");
    let col_count = galaxy_grid[0].len();

    // empty rows are simple iteration where those where all are false
    let empty_rows: Vec<usize> = galaxy_grid
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.iter().all(|x| *x == false) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();
    // iterate over known columns
    let empty_cols: Vec<usize> = (0..col_count)
        .filter_map(|col_idx| {
            // for each column index, pull the right element from every row and check if false
            if galaxy_grid.iter().map(|r| r[col_idx]).all(|x| x == false) {
                Some(col_idx)
            } else {
                None
            }
        })
        .collect();

    // add empty rows with the existing ones
    // enumeration is to add offsets for the extra positions already added
    let empty_row_vec: Vec<Vec<bool>> = vec![(0..col_count).map(|_| false).collect()];
    for (idx, i) in empty_rows.iter().enumerate() {
        galaxy_grid.splice(i + idx..i + idx, empty_row_vec.clone());
    }
    // add empty cols with the existing ones
    // enumeration is to add offsets for the extra positions already added
    for (idx, i) in empty_cols.iter().enumerate() {
        println!("Adding empty col after {i}");
        // within each one we have to splice within each row
        for row in galaxy_grid.iter_mut() {
            row.splice(i + idx..i + idx, vec![false]);
        }
    }

    // print grid to inspect
    for r in &galaxy_grid {
        println!(
            "{}",
            r.iter()
                .map(|&x| if x { "#" } else { "." })
                .collect::<Vec<&str>>()
                .join("")
        );
    }

    // collect positions
    let galaxy_positions: Vec<(usize, usize)> = galaxy_grid
        .iter()
        .enumerate()
        .flat_map(|(row_idx, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(col_idx, &c)| if c { Some((row_idx, col_idx)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let mut dists: Vec<usize> = vec![];
    for i in 0..galaxy_positions.len() {
        for j in i + 1..galaxy_positions.len() {
            let dist = galaxy_positions[i].0.abs_diff(galaxy_positions[j].0)
                + galaxy_positions[i].1.abs_diff(galaxy_positions[j].1);
            dists.push(dist)
        }
    }

    println!("Sum of distances: {}", dists.iter().sum::<usize>());
}
