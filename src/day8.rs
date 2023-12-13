use std::collections::HashMap;

fn get_day8_input() -> (Vec<usize>, HashMap<&'static str, [&'static str; 2]>) {
    let mut input_str_lines = include_str!("../examples/day8_input.txt").lines();
    let lr_instructions: Vec<usize> = input_str_lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("Did not expect direction {x}"),
        })
        .collect();

    let slot_map: HashMap<&str, [&str; 2]> = input_str_lines
        .filter(|x| x.trim() != "")
        .map(|x| {
            let split_line: Vec<&str> = x.trim().split(" = ").collect();
            let line_sides: Vec<&str> = split_line[1][1..split_line[1].len() - 1]
                .split(", ")
                .collect();
            (split_line[0], [line_sides[0], line_sides[1]])
        })
        .collect();

    (lr_instructions, slot_map)
}
pub fn day8_p1() {
    let (lr_instructions, slot_map) = get_day8_input();
    println!("{:#?}", lr_instructions);
    println!("{:#?}", slot_map);
    let mut cur_pos = "AAA";
    let mut iteration = 0;
    let mut cur_dir_idx = 0;

    while cur_pos != "ZZZ" {
        iteration += 1;
        println!("before iteration={iteration}, cur_pos={cur_pos}");
        let direction = lr_instructions[cur_dir_idx];
        cur_pos = slot_map.get(cur_pos).unwrap()[direction];
        println!("after iteration={iteration}, cur_pos={cur_pos}");
        cur_dir_idx = (cur_dir_idx + 1) % lr_instructions.len();
    }
    println!("Total steps: {iteration}");
}
