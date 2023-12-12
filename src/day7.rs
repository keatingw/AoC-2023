use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, Hash, Clone)]
enum CamelCard {
    Letter(char),
    Number(u16),
}

fn letter_rank(l: &char) -> u16 {
    match l {
        'T' => 0,
        'J' => 1,
        'Q' => 2,
        'K' => 3,
        'A' => 4,
        _ => panic!("{l} not expected"),
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // letters != numbers
            (CamelCard::Letter(_), CamelCard::Number(_)) => false,
            (CamelCard::Number(_), CamelCard::Letter(_)) => false,
            // if both same type, compare inner element
            (CamelCard::Letter(ll), CamelCard::Letter(rl)) => ll == rl,
            (CamelCard::Number(ln), CamelCard::Number(rn)) => ln == rn,
        }
    }
}
impl Eq for CamelCard {}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // letters always > numbers
            (CamelCard::Letter(_), CamelCard::Number(_)) => Ordering::Greater,
            (CamelCard::Number(_), CamelCard::Letter(_)) => Ordering::Less,
            // if both letters, need to compare backwards as letters compare opposite order
            (CamelCard::Letter(ll), CamelCard::Letter(rl)) => {
                letter_rank(&ll).cmp(&letter_rank(&rl))
            }
            // if both numbers, can compare the normal way
            (CamelCard::Number(ln), CamelCard::Number(rn)) => ln.cmp(&rn),
        }
    }
}

#[derive(Debug)]
enum CamelHand {
    HighCard([CamelCard; 5]),
    Pair([CamelCard; 5]),
    TwoPair([CamelCard; 5]),
    ThreeOfKind([CamelCard; 5]),
    FullHouse([CamelCard; 5]),
    FourOfKind([CamelCard; 5]),
    FiveOfKind([CamelCard; 5]),
}

impl CamelHand {
    fn from_cards(cards: &[CamelCard; 5]) -> Self {
        let mut frequencies: Vec<(CamelCard, u32)> = cards
            .iter()
            .fold(
                HashMap::new(),
                |mut map: HashMap<&CamelCard, u32>, val: &CamelCard| {
                    *map.entry(val).or_default() += 1;
                    map
                },
            )
            .into_iter()
            .map(|(x, y)| (x.clone(), y))
            .collect();
        // sort most -> least frequency
        frequencies.sort_by_key(|x| -(x.1 as i64));

        // escape hatch since in this case only 1 element to match on
        if frequencies[0].1 == 5 {
            return CamelHand::FiveOfKind(cards.clone());
        }

        // top two frequencies to determine hand type
        match (frequencies[0].1, frequencies[1].1) {
            (4, _) => CamelHand::FourOfKind(cards.clone()),
            (3, 2) => CamelHand::FullHouse(cards.clone()),
            (3, _) => CamelHand::ThreeOfKind(cards.clone()),
            (2, 2) => CamelHand::TwoPair(cards.clone()),
            (2, _) => CamelHand::Pair(cards.clone()),
            _ => CamelHand::HighCard(cards.clone()),
        }
    }
}
impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CamelHand::FiveOfKind(l), CamelHand::FiveOfKind(r)) => l == r,
            (CamelHand::FourOfKind(l), CamelHand::FourOfKind(r)) => l == r,
            (CamelHand::FullHouse(l), CamelHand::FullHouse(r)) => l == r,
            (CamelHand::ThreeOfKind(l), CamelHand::ThreeOfKind(r)) => l == r,
            (CamelHand::TwoPair(l), CamelHand::TwoPair(r)) => l == r,
            (CamelHand::Pair(l), CamelHand::Pair(r)) => l == r,
            (CamelHand::HighCard(l), CamelHand::HighCard(r)) => l == r,
            _ => false,
        }
    }
}
impl Eq for CamelHand {}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (CamelHand::FiveOfKind(l), CamelHand::FiveOfKind(r)) => l.cmp(r),
            (CamelHand::FiveOfKind(_), _) => Ordering::Greater,
            (CamelHand::FourOfKind(_), CamelHand::FiveOfKind(_)) => Ordering::Less,
            (CamelHand::FourOfKind(l), CamelHand::FourOfKind(r)) => l.cmp(r),
            (CamelHand::FourOfKind(_), _) => Ordering::Greater,
            (CamelHand::FullHouse(_), CamelHand::FiveOfKind(_) | CamelHand::FourOfKind(_)) => {
                Ordering::Less
            }
            (CamelHand::FullHouse(l), CamelHand::FullHouse(r)) => l.cmp(r),
            (CamelHand::FullHouse(_), _) => Ordering::Greater,
            (
                CamelHand::ThreeOfKind(_),
                CamelHand::FiveOfKind(_) | CamelHand::FourOfKind(_) | CamelHand::FullHouse(_),
            ) => Ordering::Less,
            (CamelHand::ThreeOfKind(l), CamelHand::ThreeOfKind(r)) => l.cmp(r),
            (CamelHand::ThreeOfKind(_), _) => Ordering::Greater,
            (
                CamelHand::TwoPair(_),
                CamelHand::FiveOfKind(_)
                | CamelHand::FourOfKind(_)
                | CamelHand::FullHouse(_)
                | CamelHand::ThreeOfKind(_),
            ) => Ordering::Less,
            (CamelHand::TwoPair(l), CamelHand::TwoPair(r)) => l.cmp(r),
            (CamelHand::TwoPair(_), _) => Ordering::Greater,
            (
                CamelHand::Pair(_),
                CamelHand::FiveOfKind(_)
                | CamelHand::FourOfKind(_)
                | CamelHand::FullHouse(_)
                | CamelHand::ThreeOfKind(_)
                | CamelHand::TwoPair(_),
            ) => Ordering::Less,
            (CamelHand::Pair(l), CamelHand::Pair(r)) => l.cmp(r),
            (CamelHand::Pair(_), _) => Ordering::Greater,
            (CamelHand::HighCard(l), CamelHand::HighCard(r)) => l.cmp(r),
            (CamelHand::HighCard(_), _) => Ordering::Less,
        }
    }
}

fn get_day7_input() -> Vec<([CamelCard; 5], u32)> {
    let input_str = include_str!("../examples/day7_input.txt");
    input_str
        .lines()
        .map(|x| {
            let split_elements: Vec<&str> = x.split_ascii_whitespace().collect();
            let camel_cards: [CamelCard; 5] = split_elements[0]
                .chars()
                .map(|c| {
                    if c.is_numeric() {
                        CamelCard::Number(c.to_string().parse::<u16>().unwrap())
                    } else {
                        CamelCard::Letter(c)
                    }
                })
                .collect::<Vec<CamelCard>>()
                .try_into()
                .unwrap();
            let bid = split_elements[1].parse::<u32>().unwrap();
            (camel_cards, bid)
        })
        .collect()
}

pub fn day7_p1() {
    let inputs = get_day7_input();
    // println!("{:#?}", &inputs);
    let mut hands: Vec<(CamelHand, u32)> = inputs
        .iter()
        .map(|x| (CamelHand::from_cards(&x.0), x.1))
        .collect();

    hands.sort_by(|x, y| x.0.cmp(&y.0));
    println!("sorted hands: {:#?}", hands);
    // println!("sorted hands: {:#?}", hands);
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, x)| (idx as u32 + 1) * x.1)
        .sum::<u32>();
    println!("total winnings: {:#?}", total_winnings);
}
