#[derive(Debug)]
struct Card {
    id: i32,
    winning: Vec<i32>,
    held: Vec<i32>,
}

impl Card {
    fn get_score(&self) -> i32 {
        let held_winning = self.held_winning_nums();
        match held_winning.is_empty() {
            true => 0,
            false => match held_winning.iter().len().try_into().unwrap() {
                1 => 1,
                len => 2_i32.pow(len - 1),
            },
        }
    }

    fn held_winning_nums(&self) -> Vec<i32> {
        self.winning
            .iter()
            .filter(|w| self.held.iter().find(|h| h == w).is_some())
            .copied()
            .collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let parsed_cards: Vec<Card> = input.lines().map(parse_card).collect();

    let total_points: i32 = parsed_cards.iter().map(|c| c.get_score()).sum();

    println!("The total points won is {total_points}");
}

fn parse_card(input: &str) -> Card {
    let mut split_input = input.split(':');
    let id: i32 = split_input
        .next()
        .unwrap()
        .rmatches(char::is_numeric)
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();

    let mut num_lists = split_input.last().unwrap().split('|');

    let winning: Vec<i32> = parse_num_list(num_lists.next().unwrap());
    let held: Vec<i32> = parse_num_list(num_lists.last().unwrap());

    Card { id, winning, held }
}

fn parse_num_list(num_list: &str) -> Vec<i32> {
    num_list
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zero_numeric() {
        assert!('0'.is_numeric());
    }

    #[test]
    fn calculate_score() {
        let four_matches = Card {
            id: 0,
            winning: vec![41, 48, 83, 86, 17],
            held: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let two_matches = Card {
            id: 0,
            winning: vec![13, 32, 20, 16, 61],
            held: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };

        let one_match = Card {
            id: 0,
            winning: vec![41, 92, 73, 84, 69],
            held: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };

        let no_matches = Card {
            id: 0,
            winning: vec![31, 18, 13, 56, 72],
            held: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };

        assert_eq!(four_matches.get_score(), 8);
        assert_eq!(two_matches.get_score(), 2);
        assert_eq!(one_match.get_score(), 1);
        assert_eq!(no_matches.get_score(), 0);
    }
}
