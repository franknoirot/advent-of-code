use std::num::ParseIntError;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let content_newlines = input.split('\n').filter(|s| !s.is_empty());

    let digit_tuples = content_newlines
        .clone()
        .filter_map(problem_one)
        .filter_map(parse_str_tuple)
        .filter_map(|(first, last)| format!("{first}{last}").parse::<i32>().ok());

    // digit_tuples
    //     .enumerate()
    //     .for_each(|(i, n)| println!("{i}: {:?}", n));

    println!(
        "The sum of all the valid lines' numbers is: {}",
        digit_tuples.sum::<i32>()
    );
}

fn parse_str_tuple(str_tuple: (&str, &str)) -> Option<(i32, i32)> {
    match (parse_digit_word(str_tuple.0), parse_digit_word(str_tuple.1)) {
        (Ok(first), Ok(last)) => Some((first, last)),
        (_, _) => None,
    }
}

fn parse_digit_word(word_or_digit: &str) -> Result<i32, ParseIntError> {
    match word_or_digit.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => match word_or_digit {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err(e),
        },
    }
}

fn problem_one(input: &str) -> Option<(&str, &str)> {
    let re = Regex::new(r"\d").unwrap();
    let matches: Vec<regex::Match<'_>> = re.find_iter(input).collect();

    match (matches.first(), matches.last()) {
        (Some(first_match), Some(second_match)) => {
            Some((first_match.as_str(), second_match.as_str()))
        }
        (Some(first_match), None) => Some((first_match.as_str(), first_match.as_str())),
        (None, Some(second_match)) => Some((second_match.as_str(), second_match.as_str())),
        (_, _) => None,
    }
}