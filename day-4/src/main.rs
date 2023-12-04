#[derive(Debug, Clone)]
struct Card {
    winning: Vec<i32>,
    held: Vec<i32>,
}

impl Card {
    fn get_score(&self) -> i32 {
        let held_winning = self.held_winning_nums();

        if held_winning.is_empty() {
            0
        } else {
            match held_winning.len().try_into().unwrap() {
                1 => 1,
                len => 2_i32.pow(len - 1),
            }
        }
    }

    fn held_winning_nums(&self) -> Vec<i32> {
        self.winning
            .iter()
            .filter(|w| self.held.iter().any(|h| h == *w))
            .copied()
            .collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let parsed_cards: Vec<Card> = input.lines().map(parse_card).collect();

    // Problem 1
    let total_points: i32 = parsed_cards.iter().map(|c| c.get_score()).sum();
    println!("Problem 1:\nThe total points won is {total_points}");

    // Problem 2
    let total_cards: u32 = get_total_cards_won(&parsed_cards);
    println!("\nProblem 2:\nThe total number of scratch cards won is {total_cards}");

}

fn get_total_cards_won(cards: &Vec<Card>) -> u32 {
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(i, c)| {
        if i == card_counts.len() {
            return;
        }

        let held_winning_count: usize = c.held_winning_nums().len();

        let start = (i + 1).min(card_counts.len());
        let end = (i + 1 + held_winning_count).min(card_counts.len());

        for n in start..end {
            card_counts[n] += card_counts[i];
        }
    });

    card_counts.iter().sum()
}

fn parse_card(input: &str) -> Card {
    let split_input = input.split(':');
    let mut num_lists = split_input.last().unwrap().split('|');

    let winning: Vec<i32> = parse_num_list(num_lists.next().unwrap());
    let held: Vec<i32> = parse_num_list(num_lists.last().unwrap());

    Card { winning, held }
}

fn parse_num_list(num_list: &str) -> Vec<i32> {
    num_list
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
            winning: vec![41, 48, 83, 86, 17],
            held: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let two_matches = Card {
            winning: vec![13, 32, 20, 16, 61],
            held: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };

        let one_match = Card {
            winning: vec![41, 92, 73, 84, 69],
            held: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };

        let no_matches = Card {
            winning: vec![31, 18, 13, 56, 72],
            held: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };

        assert_eq!(four_matches.get_score(), 8);
        assert_eq!(two_matches.get_score(), 2);
        assert_eq!(one_match.get_score(), 1);
        assert_eq!(no_matches.get_score(), 0);
    }

    #[test]
    fn test_winning_counts() {
        let test_input: &str = include_str!("../test_input.txt");
        let test_parsed_cards: Vec<Card> = test_input.lines().map(parse_card).collect();

        assert_eq!(
            test_parsed_cards
                .iter()
                .map(|c| c.held_winning_nums().len())
                .collect::<Vec<usize>>(),
            vec![4, 2, 2, 1, 0, 0]
        );
    }

    #[test]
    fn test_total_cards() {
        let test_input: &str = include_str!("../test_input.txt");
        let test_parsed_cards: Vec<Card> = test_input.lines().map(parse_card).collect();

        assert_eq!(get_total_cards_won(&test_parsed_cards), 30);
    }
}
