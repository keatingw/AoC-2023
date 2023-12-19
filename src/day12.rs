use rayon::prelude::*;
use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecordType {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRecordError;

impl FromStr for RecordType {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() > 1 {
            return Err(ParseRecordError);
        }
        let mut chars = s.chars();
        match chars.next() {
            Some('#') => Ok(RecordType::Damaged),
            Some('.') => Ok(RecordType::Operational),
            Some('?') => Ok(RecordType::Unknown),
            _ => Err(ParseRecordError),
        }
    }
}

fn get_day12_input(path: &str) -> Vec<(Vec<RecordType>, Vec<usize>)> {
    let input_str = fs::read_to_string(path).unwrap();
    let input_groups: Vec<[String; 2]> = input_str
        .lines()
        .map(|l| {
            l.split(" ")
                .map(String::from)
                .collect::<Vec<String>>()
                .try_into()
                .unwrap()
        })
        .collect();
    input_groups
        .iter()
        .map(|x| {
            let spring_records: Vec<RecordType> = x[0]
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            let contig_groups: Vec<usize> = x[1].split(",").map(|c| c.parse().unwrap()).collect();
            (spring_records, contig_groups)
        })
        .collect()
}

fn count_groups(records: &[RecordType], groups: &[usize]) -> usize {
    if records.len() == 0 {
        // if no records left but still groups, fail
        if groups.len() > 0 {
            return 0;
        // if no records left and no groups, succeed!
        } else {
            return 1;
        }
    }
    if groups.len() == 0 {
        // if no groups but more damaged still exist then no ways to make it
        if records.iter().any(|x| *x == RecordType::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }

    // main cases: need to run through rest of list
    // start accumulator
    let mut result: usize = 0;

    // if we're operational or treating this unknown as optional, skip a step
    if let RecordType::Operational | RecordType::Unknown = records[0] {
        result += count_groups(&records[1..], groups);
    }

    // if we're damaged or treating this unknown as damaged, try to build a group
    if let RecordType::Damaged | RecordType::Unknown = records[0] {
        // if we have enough elements left to make it in theory
        if groups[0] <= records.len()
            // *and* all the next N items are non-operational (so can make the group)
            && records[..groups[0]]
                .iter()
                .all(|x| *x != RecordType::Operational)
            // *and* we either will exactly hit the end, or the next item is possibly non-damaged
            && (groups[0] == records.len() || records[groups[0]] != RecordType::Damaged)
        {
            // when we hit the end of the list, pass an empty record list
            if groups[0] == records.len() {
                result += count_groups(&vec![], &groups[1..])
            } else {
                // otherwise skip a group and continue
                result += count_groups(&records[groups[0] + 1..], &groups[1..])
            }
        }
    }

    result
}

pub fn day12_p1() {
    let input = get_day12_input("examples/day12_input.txt");
    println!("{:#?}", input);
    let mut cumsum = 0;
    for (idx, (records, groups)) in input.iter().enumerate() {
        let count = count_groups(records, groups);
        println!("idx: {idx}, count: {count}");
        cumsum += count;
    }
    println!("Total: {cumsum}");
}

pub fn day12_p2() {
    let input = get_day12_input("examples/day12_input.txt");
    println!("{:#?}", input);
    let counts: Vec<usize> = input
        .par_iter()
        .enumerate()
        .map(|(idx, (records, groups))| {
            let mut unfolded_records: Vec<RecordType> = vec![];
            let mut unfolded_groups: Vec<usize> = vec![];
            for idx in 1..=5 {
                for i in records {
                    unfolded_records.push(i.clone());
                }
                if idx != 5 {
                    unfolded_records.push(RecordType::Unknown);
                }
                for i in groups {
                    unfolded_groups.push(i.clone());
                }
            }
            let count = count_groups(&unfolded_records, &unfolded_groups);
            println!("idx: {idx}, count: {count}");
            count
        })
        .collect();
    println!("Total: {}", counts.iter().sum::<usize>());
}
