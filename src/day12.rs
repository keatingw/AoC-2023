use std::{fs, str::FromStr};

#[derive(Debug)]
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

fn calc_damaged_groups(records: &[RecordType]) -> Vec<usize> {
    let mut contiguous_groups: Vec<usize> = vec![];
    let mut group_size: usize = 0;
    for (idx, i) in records.iter().enumerate() {
        if let RecordType::Damaged = i {
            group_size += 1;
            // if at end of list and non-empty group then add to list
            if idx == records.len() - 1 && group_size > 0 {
                contiguous_groups.push(group_size.clone());
            }
        } else {
            if group_size > 0 {
                contiguous_groups.push(group_size.clone());
                group_size = 0;
            }
        }
    }
    contiguous_groups
}

pub fn day12_p1() {
    let input = get_day12_input("examples/day12_example.txt");
    println!("{:#?}", input);
}

#[cfg(test)]
mod tests {
    use crate::day12::{calc_damaged_groups, RecordType};
    #[test]
    fn damaged_groups() {
        let records = vec![
            RecordType::Damaged,
            RecordType::Unknown,
            RecordType::Unknown,
            RecordType::Damaged,
            RecordType::Damaged,
            RecordType::Operational,
            RecordType::Damaged,
        ];
        assert_eq!(calc_damaged_groups(&records), vec![1, 2, 1])
    }
}
