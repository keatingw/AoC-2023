#[derive(Debug)]
pub struct NumericString {
    pub number: u32,
    pub row: u32,
    pub col_start: u32,
    pub length: u32,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct SymbolLocation(pub u32, pub u32);

#[derive(Debug)]
pub struct GearDetails {
    pub row: u32,
    pub col: u32,
    pub adjacent_numbers: Vec<u32>,
}

pub fn day3_p1() {
    let input_string = include_str!("../examples/day3_input.txt");

    // first collect all numbers and their positions
    let mut numeric_strings: Vec<NumericString> = Vec::new();
    // plus all symbol positions
    let mut symbol_positions: Vec<SymbolLocation> = Vec::new();
    // iterate over rows/lines, track row idx
    for (r_idx, r) in input_string.lines().enumerate() {
        // iterate over chars in line
        let mut cur_numeric: Vec<char> = Vec::new();
        for (c_idx, c) in r.chars().enumerate() {
            if !(c.is_digit(10) || c == '.') {
                symbol_positions.push(SymbolLocation(r_idx as u32, c_idx as u32));
            } else if c.is_digit(10) {
                cur_numeric.push(c);
            }
            if !cur_numeric.is_empty() && (!c.is_digit(10) || c_idx == (r.len() - 1)) {
                let len = cur_numeric.len() as u32;
                numeric_strings.push(NumericString {
                    number: cur_numeric
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                    row: r_idx as u32,
                    col_start: c_idx as u32 - len,
                    length: len,
                    enabled: false,
                });
                cur_numeric = Vec::new();
            }
        }
    }
    for sl in &symbol_positions {
        for num in &mut numeric_strings {
            if num.enabled {
                continue;
            }
            if num.row == sl.0 - 1 || num.row == sl.0 || num.row == sl.0 + 1 {
                // if row adjacent, runs within a diagonal
                for offset in 0..num.length {
                    let position = num.col_start + offset;
                    if position == sl.1 - 1 || position == sl.1 || position == sl.1 + 1 {
                        num.enabled = true;
                        break;
                    }
                }
            }
        }
    }
    let adjacent_sum = numeric_strings
        .iter()
        .filter(|x| x.enabled)
        .map(|x| x.number)
        .sum::<u32>();
    println!("Adjacent values sum: {:#?}", adjacent_sum);
}

pub fn day3_p2() {
    let input_string = include_str!("../examples/day3_input.txt");

    // first collect all numbers and their positions
    let mut numeric_strings: Vec<NumericString> = Vec::new();
    // plus all gears
    let mut gears: Vec<GearDetails> = Vec::new();
    // iterate over rows/lines, track row idx
    for (r_idx, r) in input_string.lines().enumerate() {
        // iterate over chars in line
        let mut cur_numeric: Vec<char> = Vec::new();
        for (c_idx, c) in r.chars().enumerate() {
            if c == '*' {
                gears.push(GearDetails {
                    row: r_idx as u32,
                    col: c_idx as u32,
                    adjacent_numbers: Vec::new(),
                });
            } else if c.is_digit(10) {
                cur_numeric.push(c);
            }
            if !cur_numeric.is_empty() && (!c.is_digit(10) || c_idx == (r.len() - 1)) {
                let len = cur_numeric.len() as u32;
                numeric_strings.push(NumericString {
                    number: cur_numeric
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                    row: r_idx as u32,
                    col_start: c_idx as u32 - len,
                    length: len,
                    enabled: false,
                });
                cur_numeric = Vec::new();
            }
        }
    }
    for g in &mut gears {
        for num in &mut numeric_strings {
            if num.row == g.row - 1 || num.row == g.row || num.row == g.row + 1 {
                // if row adjacent, runs within a diagonal
                for offset in 0..num.length {
                    let position = num.col_start + offset;
                    if position == g.col - 1 || position == g.col || position == g.col + 1 {
                        g.adjacent_numbers.push(num.number);
                        break;
                    }
                }
            }
        }
    }
    let ratio_sum: u32 = gears
        .iter()
        .filter(|x| x.adjacent_numbers.len() == 2)
        .map(|x| x.adjacent_numbers.iter().product::<u32>())
        .sum();

    println!("Gear ratio sum: {:#?}", ratio_sum);
}
