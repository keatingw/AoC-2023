fn get_day9_input() -> Vec<Vec<i32>> {
    let input_str: Vec<Vec<i32>> = include_str!("../examples/day9_input.txt")
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|c| c.parse::<i32>().expect("Couldn't parse value"))
                .collect::<Vec<i32>>()
        })
        .collect();
    input_str
}

pub fn day9_p1() {
    let inputs = get_day9_input();
    let mut sum_preds = 0;
    for l in inputs {
        let mut differenced_vecs: Vec<Vec<i32>> = vec![l.clone()];
        loop {
            let diffs: Vec<i32> = differenced_vecs
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect();
            differenced_vecs.push(diffs.clone());

            // once all are 0, stop looping
            if diffs.iter().filter(|x| x != &&0).count() == 0 {
                break;
            }
        }
        // sum over the last element of each differenced vector
        let sum_pred = differenced_vecs
            .iter()
            .map(|x| x.last().unwrap())
            .sum::<i32>();
        sum_preds += sum_pred;
    }
    println!("Sum of predictions: {}", sum_preds);
}

pub fn day9_p2() {
    let inputs = get_day9_input();
    let mut sum_preds = 0;
    for l in inputs {
        let mut differenced_vecs: Vec<Vec<i32>> = vec![l.clone()];
        loop {
            let diffs: Vec<i32> = differenced_vecs
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect();
            differenced_vecs.push(diffs.clone());

            // once all are 0, stop looping
            if diffs.iter().filter(|x| x != &&0).count() == 0 {
                break;
            }
        }
        let sum_pred = differenced_vecs
            .iter()
            // take start of each differenced vec
            .map(|x| x.first().unwrap())
            // reverse so we start from the last set
            .rev()
            // fold together, subtracting from the top
            .fold(0, |a, x| x - a);
        sum_preds += sum_pred
    }
    println!("Sum of predictions: {}", sum_preds);
}
