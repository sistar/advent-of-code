use std::collections::HashSet;

use itertools::Itertools;

use crate::custom_error::AocError;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandTypes {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

fn char_to_card(c: char) -> Cards {
    match c {
        'K' => Cards::K,
        'A' => Cards::A,
        'Q' => Cards::Q,
        'J' => Cards::J,
        'T' => Cards::T,
        '9' => Cards::Nine,
        '8' => Cards::Eight,
        '7' => Cards::Seven,
        '6' => Cards::Six,
        '5' => Cards::Five,
        '4' => Cards::Four,
        '3' => Cards::Three,
        '2' => Cards::Two,
        _ => panic!("Invalid card character"),
    }
}
#[derive(Debug)]
struct Hand {
    cards: Vec<Cards>,
    hand_type: HandTypes,
}

impl Hand {
    fn new(cards: Vec<Cards>) -> Hand {
        if cards.len() != 5 {
            panic!("Invalid number of cards");
        }
        let hand_type = Hand::get_hand_type(&cards);
        Hand {
            cards,
            hand_type: hand_type,
        }
    }
    fn from_str(input: &str) -> Hand {
        let mut cards = Vec::new();
        for c in input.chars() {
            cards.push(char_to_card(c));
        }
        Hand::new(cards)
    }

    fn get_hand_type(cards: &[Cards]) -> HandTypes {
        let cards_set: HashSet<_> = cards.iter().collect();
        let cards_set_len = cards_set.len();
        let mut counts: Vec<usize> = cards_set
            .iter()
            .map(|c| cards.iter().filter(|c2| c2 == c).count())
            .collect::<Vec<usize>>();
        counts.sort();
        //dbg!(cards_set_len, counts);
        match cards_set_len {
            1 if counts == vec![5] => HandTypes::FiveOfAKind,
            2 if counts == vec![1, 4] => HandTypes::FourOfAKind,
            2 if counts == vec![2, 3] => HandTypes::FullHouse,
            3 if counts == vec![1, 1, 3] => HandTypes::ThreeOfAKind,
            3 if counts == vec![1, 2, 2] => HandTypes::TwoPair,
            4 if counts == vec![1, 1, 1, 2] => HandTypes::OnePair,
            5 if counts == vec![1, 1, 1, 1, 1] => HandTypes::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = self.hand_type.cmp(&other.hand_type);
        match cmp {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(c1, c2)| c1.cmp(c2) != std::cmp::Ordering::Equal)
                .map(|(c1, c2)| Some(c1.cmp(c2)))
                .flatten(),
            _ => Some(cmp),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_hands(input: &str) -> Vec<(Hand, &str)> {
    input
        .lines()
        .flat_map(|s| s.split_once(' '))
        .map(|(hand, bid)| (Hand::from_str(hand), bid))
        .sorted()
        .collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let hands: Vec<_> = parse_hands(input);

    let total_winnings = hands
        .iter()
        .map(|(_, bid)| bid.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, bid)| ((i as i32 + 1) * bid))
        .sum::<i32>();
    Ok(total_winnings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hand_type() {
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::A, Cards::A, Cards::A]),
            HandTypes::FiveOfAKind
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::A, Cards::A, Cards::K]),
            HandTypes::FourOfAKind
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::A, Cards::K, Cards::K]),
            HandTypes::FullHouse
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::A, Cards::K, Cards::Q]),
            HandTypes::ThreeOfAKind
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::K, Cards::K, Cards::Q]),
            HandTypes::TwoPair
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::A, Cards::K, Cards::Q, Cards::J]),
            HandTypes::OnePair
        );
        assert_eq!(
            Hand::get_hand_type(&vec![Cards::A, Cards::K, Cards::Q, Cards::J, Cards::T]),
            HandTypes::HighCard
        );
    }
    #[test]
    fn test_hand_from_str() {
        assert_eq!(Hand::from_str("32T3K").hand_type, HandTypes::OnePair);
    }

    #[test]
    fn test_hand_order() {
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKKK"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("AAAAK"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("AAAAT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKKQ"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKKT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKQQ"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKQT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKKTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKQQQ"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKQQT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KKQTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KQQQT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KQQTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("KQTTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QQQQJ"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QQQJT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QQQJJ"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QQQTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QQJTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("QJJTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("JJJTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("JJTTT"));
        assert!(Hand::from_str("AAAAA") > Hand::from_str("TTTT9"));
        assert!(Hand::from_str("33332") > Hand::from_str("2AAAA"));
    }

    #[test]
    fn test_parse_hands() -> miette::Result<()> {
        let input = include_str!("../sample.txt");
        let binding = parse_hands(input);
        let hands = binding.iter().map(|(h, _)| h).collect_vec();
        assert_eq!(*hands[0], Hand::from_str("32T3K"));
        assert_eq!(*hands[1], Hand::from_str("KTJJT"));
        assert_eq!(*hands[2], Hand::from_str("KK677"));
        assert_eq!(*hands[3], Hand::from_str("T55J5"));
        println!(" EQUAL {:?} {:?}", hands[4], Hand::from_str("T55J5"));
        assert_eq!(*hands[4], Hand::from_str("T55J5"));
        Ok(())
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../sample.txt");
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
