use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;

enum Part {
    One,
    Two,
}
#[derive(Debug, Hash, Clone)]
enum CamelCard {
    LetterP1(char),
    Number(u16),
    LetterP2(char),
}

impl CamelCard {
    fn card_rank(&self) -> u16 {
        match self {
            CamelCard::LetterP1('T') => 10,
            CamelCard::LetterP1('J') => 11,
            CamelCard::LetterP1('Q') => 12,
            CamelCard::LetterP1('K') => 13,
            CamelCard::LetterP1('A') => 14,
            CamelCard::LetterP2('T') => 10,
            CamelCard::LetterP2('J') => 1,
            CamelCard::LetterP2('Q') => 12,
            CamelCard::LetterP2('K') => 13,
            CamelCard::LetterP2('A') => 14,
            CamelCard::Number(x) => x.clone(),
            _ => panic!("Did not expect card value {self:#?}"),
        }
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.card_rank() == other.card_rank()
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
        self.card_rank().cmp(&other.card_rank())
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    fn from_cards_p1(cards: &[CamelCard; 5]) -> Self {
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

    fn from_cards_p2(cards: &[CamelCard; 5]) -> Self {
        let mut frequencies: Vec<(CamelCard, u32)> = cards
            .iter()
            .filter(|x| x != &&CamelCard::LetterP2('J'))
            .fold(
                HashMap::new(),
                |mut map: HashMap<&CamelCard, u32>, val: &CamelCard| {
                    *map.entry(val).or_insert(0) += 1;
                    map
                },
            )
            .into_iter()
            .map(|(x, y)| (x.clone(), y))
            .collect();
        let num_jokers = cards
            .iter()
            .filter(|x| x == &&CamelCard::LetterP2('J'))
            .count() as u32;

        // sort most -> least frequency
        frequencies.sort_by_key(|x| -(x.1 as i64));

        // top frequencies to determine hand type
        match if frequencies.len() > 0 {
            frequencies[0].1 + num_jokers
        } else {
            // fallback if all were jokers
            num_jokers
        } {
            5 => CamelHand::FiveOfKind(cards.clone()),
            4 => CamelHand::FourOfKind(cards.clone()),
            3 => {
                if frequencies[1].1 == 2 {
                    CamelHand::FullHouse(cards.clone())
                } else {
                    CamelHand::ThreeOfKind(cards.clone())
                }
            }
            2 => {
                if frequencies[1].1 == 2 {
                    CamelHand::TwoPair(cards.clone())
                } else {
                    CamelHand::Pair(cards.clone())
                }
            }
            _ => CamelHand::HighCard(cards.clone()),
        }
    }
}

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

fn get_day7_input(part: Part) -> Vec<([CamelCard; 5], u32)> {
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
                        match part {
                            Part::One => CamelCard::LetterP1(c),
                            Part::Two => CamelCard::LetterP2(c),
                        }
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
    let inputs = get_day7_input(Part::One);
    let mut hands: Vec<(CamelHand, u32)> = inputs
        .iter()
        .map(|x| (CamelHand::from_cards_p1(&x.0), x.1))
        .collect();

    hands.sort_by(|x, y| x.0.cmp(&y.0));
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, x)| (idx as u32 + 1) * x.1)
        .sum::<u32>();
    println!("total winnings: {:#?}", total_winnings);
}

pub fn day7_p2() {
    let inputs = get_day7_input(Part::Two);
    let mut hands: Vec<(CamelHand, u32)> = inputs
        .iter()
        .map(|x| (CamelHand::from_cards_p2(&x.0), x.1))
        .collect();

    hands.sort_by(|x, y| x.0.cmp(&y.0));
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, x)| (idx as u32 + 1) * x.1)
        .sum::<u32>();
    println!("total winnings: {:#?}", total_winnings);
}
