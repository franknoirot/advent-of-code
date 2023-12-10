use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: String,
    bet: i64,
    hand_type: HandType,
}

#[derive(Debug, Eq, Clone)]
struct JokerfiedHand {
    cards: String,
    bet: i64,
    hand_type: HandType,
}

impl From<Hand> for JokerfiedHand {
    fn from(value: Hand) -> Self {
        JokerfiedHand {
            cards: value.cards,
            bet: value.bet,
            hand_type: value.hand_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

const CARD_POWERS: &str = "AKQJT98765432";
const JOKERFIED_CARD_POWERS: &str = "AKQT98765432J";

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialEq for JokerfiedHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.partial_cmp(&other.hand_type).unwrap()
        } else {
            compare_hand_strs(&self.cards, &other.cards, CARD_POWERS)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerfiedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.partial_cmp(&other.hand_type).unwrap()
        } else {
            compare_hand_strs(&self.cards, &other.cards, JOKERFIED_CARD_POWERS)
        }
    }
}

impl PartialOrd for JokerfiedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_hand_strs(first: &str, second: &str, lookup_str: &str) -> Ordering {
    let cards_zipped = first.chars().zip(second.chars());

    for (a, b) in cards_zipped {
        match (lookup_str.find(a), lookup_str.find(b)) {
            (Some(s), Some(o)) => {
                if s == o {
                    continue;
                } else {
                    return s.cmp(&o);
                }
            }
            p => panic!("Invalid card pair {:?} provided", p),
        }
    }

    Ordering::Equal
}

fn main() {
    let input = include_str!("../input.txt");

    let hands = input.lines().map(parse_hand).sorted();
    let first_score = hands
        .clone()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + hand.bet * (i as i64 + 1));

    let jokerfied_hands = input.lines().map(jokerfied_parse_hand).sorted();
    let second_score = jokerfied_hands
        .rev()
        .enumerate()
        // .for_each(|(i, h)| println!("{i}: {:?}", h));
        .fold(0, |sum, (i, hand)| sum + hand.bet * (i as i64 + 1));

    println!("Answer 1: {first_score}");
    println!("Answer 2: {second_score}");
}

fn parse_hand(hand_str: &str) -> Hand {
    let mut split = hand_str.split_whitespace();
    let cards = split.next().unwrap().to_owned();
    let bet: i64 = split.next().unwrap().parse().unwrap();
    let hand_type: HandType = get_hand_type(get_card_counts(&cards));

    Hand {
        cards,
        bet,
        hand_type,
    }
}

fn jokerfied_parse_hand(hand_str: &str) -> JokerfiedHand {
    let mut split = hand_str.split_whitespace();
    let cards = split.next().unwrap().to_owned();
    let bet: i64 = split.next().unwrap().parse().unwrap();
    let hand_type: HandType = jokerfied_get_hand_type(get_card_counts(&cards));

    JokerfiedHand {
        cards,
        bet,
        hand_type,
    }
}

fn get_card_counts(hand: &str) -> Vec<(char, i64)> {
    hand.chars()
        .sorted()
        .map(|c| (c, 1))
        .coalesce(|prev, curr| {
            if prev.0 == curr.0 {
                Ok((prev.0, prev.1 + curr.1))
            } else {
                Err((prev, curr))
            }
        })
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .collect_vec()
}

fn get_hand_type(hand: Vec<(char, i64)>) -> HandType {
    let first_match = hand.first();
    let first_two_matches = hand
        .iter()
        .take(2)
        .collect_tuple::<(&(char, i64), &(char, i64))>();

    if first_match.is_some() && first_two_matches.is_none() {
        return HandType::FiveOfKind;
    } else if first_two_matches.is_none() {
        return HandType::HighCard;
    }

    match first_two_matches.unwrap() {
        ((_, 4), _) => HandType::FourOfKind,
        ((_, 3), (_, 2)) => HandType::FullHouse,
        ((_, 3), _) => HandType::ThreeOfKind,
        ((_, 2), (_, 2)) => HandType::TwoPair,
        ((_, 2), _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn jokerfied_get_hand_type(hand: Vec<(char, i64)>) -> HandType {
    let num_jokers = hand.iter().find(|(c, _)| *c == 'J').unwrap_or(&('J', 0)).1;
    let first_match = hand.first();
    let first_two_matches = hand
        .iter()
        .take(2)
        .collect_tuple::<(&(char, i64), &(char, i64))>();

    if first_match.is_some() && first_two_matches.is_none() {
        return HandType::FiveOfKind;
    } else if first_two_matches.is_none() {
        return HandType::HighCard;
    }

    match first_two_matches.unwrap() {
        ((_, 4), _) => match num_jokers {
            0 => HandType::FourOfKind,
            _ => HandType::FiveOfKind,
        },
        ((_, 3), (_, 2)) => match num_jokers {
            3 => HandType::FiveOfKind,
            2 => HandType::FiveOfKind,
            1 => HandType::FourOfKind,
            _ => HandType::FullHouse,
        },
        ((c, 3), _) => {
            if *c == 'J' {
                return HandType::FourOfKind;
            }
            match num_jokers {
                2 => HandType::FiveOfKind,
                1 => HandType::FourOfKind,
                _ => HandType::ThreeOfKind,
            }
        }
        ((a, 2), (b, 2)) => {
            if *a == 'J' || *b == 'J' {
                return HandType::FourOfKind;
            }
            match num_jokers {
                1 => HandType::FullHouse,
                _ => HandType::TwoPair,
            }
        }
        ((c, 2), _) => {
            if *c == 'J' {
                return HandType::ThreeOfKind;
            }
            match num_jokers {
                3 => HandType::FiveOfKind,
                2 => HandType::FourOfKind,
                1 => HandType::ThreeOfKind,
                _ => HandType::OnePair,
            }
        }
        _ => {
            if num_jokers > 0 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_hand() {
        let test = "32T3K 76";
        assert_eq!(
            parse_hand(test),
            Hand {
                cards: String::from("32T3K"),
                bet: 76,
                hand_type: HandType::OnePair
            }
        );

        let test_two = "T55J5 0";
        assert_eq!(
            parse_hand(test_two),
            Hand {
                cards: String::from("T55J5"),
                bet: 0,
                hand_type: HandType::ThreeOfKind
            }
        );

        let test_two = "JJJJJ 0";
        assert_eq!(
            parse_hand(test_two),
            Hand {
                cards: String::from("JJJJJ"),
                bet: 0,
                hand_type: HandType::FiveOfKind
            }
        );
    }

    #[test]
    fn test_card_counts() {
        let test = vec![('3', 2), ('2', 1), ('K', 1), ('T', 1)];
        assert_eq!(test, get_card_counts("32T3K"))
    }

    #[test]
    fn test_card_type() {
        assert_eq!(
            HandType::OnePair,
            get_hand_type(vec![('3', 2), ('2', 1), ('K', 1), ('T', 1)])
        );
        assert_eq!(HandType::FullHouse, get_hand_type(vec![('A', 3), ('Q', 2)]));
    }

    #[test]
    fn hand_type_ordering() {
        assert!(HandType::FullHouse < HandType::OnePair);
        assert!(HandType::ThreeOfKind < HandType::HighCard);
        assert!(HandType::FiveOfKind < HandType::FourOfKind);
    }

    #[test]
    fn hand_ordering() {
        assert!(parse_hand("32T3K 0") > parse_hand("T55J5 0"));
        assert!(parse_hand("T55J5 0") > parse_hand("QQQJA 0"));
        assert!(parse_hand("QQQTQ 0") > parse_hand("QQQQ4 0"));
    }

    #[test]
    fn lookup_comparisons() {
        assert_eq!(
            compare_hand_strs("QQQTQ", "QQQQ4", CARD_POWERS),
            Ordering::Greater
        );
        assert_eq!(
            compare_hand_strs("33332", "2AAAA", CARD_POWERS),
            Ordering::Less
        );
        assert_eq!(
            compare_hand_strs("77888", "77788", CARD_POWERS),
            Ordering::Less
        );
        assert_eq!(
            compare_hand_strs("QQQJA", "KTJJT", CARD_POWERS),
            Ordering::Greater
        );
        assert_eq!(
            compare_hand_strs("JQQJA", "KTJJT", CARD_POWERS),
            Ordering::Greater
        );
    }

    #[test]
    fn jokerfied_lookup_comparisons() {
        assert_eq!(
            compare_hand_strs("QQQTQ", "QQQQ4", JOKERFIED_CARD_POWERS),
            Ordering::Greater
        );
        assert_eq!(
            compare_hand_strs("33332", "2AAAA", JOKERFIED_CARD_POWERS),
            Ordering::Less
        );
        assert_eq!(
            compare_hand_strs("77888", "77788", JOKERFIED_CARD_POWERS),
            Ordering::Less
        );
        assert_eq!(
            compare_hand_strs("QQQJA", "KTJJT", JOKERFIED_CARD_POWERS),
            Ordering::Greater
        );
        assert_eq!(
            compare_hand_strs("JQQJA", "KTJJT", JOKERFIED_CARD_POWERS),
            Ordering::Greater
        );
    }

    #[test]
    fn joker_card_types() {
        let test_one = vec![('3', 2), ('2', 1), ('K', 1), ('T', 1)];
        assert_eq!(jokerfied_get_hand_type(test_one), HandType::OnePair);

        let test_two = vec![('5', 3), ('J', 1), ('T', 1)];
        assert_eq!(jokerfied_get_hand_type(test_two), HandType::FourOfKind);

        assert_eq!(
            jokerfied_parse_hand("JJQJK 0").hand_type,
            HandType::FourOfKind
        );

        assert_eq!(
            jokerfied_parse_hand("JJ8JJ 0").hand_type,
            HandType::FiveOfKind
        );
    }
}
