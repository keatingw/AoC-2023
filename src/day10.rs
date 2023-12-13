use std::collections::HashSet;

#[derive(Debug)]
struct Pipe {
    c: char,
    x: i32,
    y: i32,
    directions: Vec<(i32, i32)>,
}
const NORTH: (i32, i32) = (-1, 0);
const SOUTH: (i32, i32) = (1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

impl Pipe {
    fn from_letter(x: i32, y: i32, c: char) -> Self {
        let directions = match c {
            '|' => vec![NORTH, SOUTH],
            '-' => vec![EAST, WEST],
            'L' => vec![NORTH, EAST],
            'J' => vec![NORTH, WEST],
            '7' => vec![SOUTH, WEST],
            'F' => vec![SOUTH, EAST],
            '.' => vec![],
            'S' => vec![NORTH, SOUTH, EAST, WEST],
            _ => panic!("Didn't expect character {c}"),
        };
        Pipe {
            c,
            x,
            y,
            directions,
        }
    }

    fn connections(&self, row_range: &(i32, i32), col_range: &(i32, i32)) -> Vec<(i32, i32)> {
        self.directions
            .iter()
            .map(|(r, c)| (self.x + r, self.y + c))
            .filter(|(x, y)| {
                row_range.0 <= *x && *x < row_range.1 && col_range.0 <= *y && *y < col_range.1
            })
            .collect()
    }
    fn coord(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn get_day10_input() -> Vec<Vec<Pipe>> {
    let input_str = include_str!("../examples/day10_input.txt");
    let pipes: Vec<Vec<Pipe>> = input_str
        .lines()
        .enumerate()
        .map(|(row, x)| {
            x.chars()
                .enumerate()
                .map(|(col, c)| Pipe::from_letter(row as i32, col as i32, c))
                .collect::<Vec<Pipe>>()
        })
        .collect();
    pipes
}

pub fn day10_p1() {
    let inputs = get_day10_input();
    let start_point = inputs
        .iter()
        .flatten()
        .filter(|x| x.c == 'S')
        .next()
        .unwrap();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let row_range: (i32, i32) = (0, inputs.len() as i32);
    let col_range: (i32, i32) = (0, inputs[0].len() as i32);

    let mut cur_point = start_point
        // check all valid connections from the start point
        .connections(&row_range, &col_range)
        .iter()
        // pull out each connection from the inputs
        .map(|x| &inputs[x.0 as usize][x.1 as usize])
        // filter down to only those that ALSO connect back to start
        .filter(|x| {
            x.connections(&row_range, &col_range)
                .contains(&(start_point.x, start_point.y))
        })
        .next()
        .unwrap();

    let mut num_steps = 0;
    // loop till we get back to start
    loop {
        println!("Visiting {} at {:?}", cur_point.c, cur_point.coord());
        num_steps += 1;
        // add current point to visited set
        visited.insert(cur_point.coord());

        // find connections not already visited
        let new_connections: Vec<(i32, i32)> = cur_point
            .connections(&row_range, &col_range)
            .into_iter()
            .filter(|x| !(visited.contains(x) || &inputs[x.0 as usize][x.1 as usize].c == &'.'))
            .collect();

        if new_connections.len() == 1
            && (new_connections[0].0, new_connections[0].1) == start_point.coord()
        {
            println!("Reached origin after {num_steps} steps");
            break;
        }
        let new_connections: Vec<(i32, i32)> = new_connections
            .into_iter()
            .filter(|x| *x != start_point.coord())
            .collect();

        cur_point = &inputs[new_connections[0].0 as usize][new_connections[0].1 as usize];
    }

    // Maximum distance away is floordiv of total loop steps, +1 if odd number total
    let max_dist = (num_steps / 2) + if num_steps % 2 == 0 { 0 } else { 1 };
    println!("Maximum distance: {max_dist}");
}
