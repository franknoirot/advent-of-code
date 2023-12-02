use regex::Regex;
use std::fs;

const INPUT: &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(INPUT).expect("No file found");
    let content_newlines = contents.split('\n').filter(|s| !s.is_empty());

    let digit_tuples = content_newlines
        .filter_map(get_outer_digits)
        .filter_map(|(first, last)| format!("{first}{last}").parse::<i32>().ok());
        // .enumerate().for_each(|(i, n)| println!("{i}: {n}"));

    println!("The sum of all the valid lines' numbers is: {}", digit_tuples.sum::<i32>());
}

fn get_outer_digits(input: &str) -> Option<(&str, &str)> {
    // Create a Regex that captures two digits, with zero or more non-digits outside,
    // and zero or more of any digit between them.
    let re = Regex::new(r"\D*(\d)(.*(\d)\D*)?").unwrap();
    let captures = re.captures(input)?;

    match (captures.get(1), captures.get(3)) {
        (Some(first_match), Some(second_match)) => {
            Some((first_match.as_str(), second_match.as_str()))
        }
        (Some(first_match), None) => Some((first_match.as_str(), first_match.as_str())),
        (None, Some(second_match)) => Some((second_match.as_str(), second_match.as_str())),
        (_, _) => None,
    }
}
