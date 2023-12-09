use std::fmt;

struct EngineDiagram(String);

impl fmt::Binary for EngineDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = &self.0;

        let binary_str = val
            .chars()
            .map(|c| match is_symbol(c) {
                true => '1',
                false => '0',
            })
            .collect::<String>();

        fmt::Display::fmt(&binary_str, f)
    }
}

fn main() {
    let input = include_str!("../input_easier.txt");
    let width = input.lines().next().unwrap().len();

    // 1. create a string of the puzzle chars
    let no_newlines = EngineDiagram(input.lines().collect::<Vec<&str>>().join(""));
    // 2. mask out everything that isn't a symbol with 0s, symbols with 1s
    let bin_str_symbols = format!("{no_newlines:b}");
    // 3. replace all 0s offset from all 1s by original width of puzzle with 1s (in both directions)
    // 4. replace all 0s on either side of all 1s with 1s
    let mask = rotated_or_str(&rotated_or_str(&bin_str_symbols, width), 1);
    // 5. test all numbers in puzzle to see if they touch a 1 in the mask
    // 6. sum all numbers that pass that test

    println!("Size is {width}");
    println!("{mask}");
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn rotated_or_str(input: &String, offset: usize) -> String {
    // split characters into vector
    let input_vec: Vec<char> = input.chars().collect();
    let mut left = input_vec.clone();
    let mut right = input_vec.clone();

    left.rotate_left(offset);
    right.rotate_right(offset);

    input_vec
        .iter()
        .zip(left.iter().zip(right.iter()))
        .map(|(o, (l, r))| {
            if o == &'1' || l == &'1' || r == &'1' {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_tests() {
        assert!(is_symbol('#'));
        assert!(is_symbol('a'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('3'));
        assert!(is_symbol('?'));
    }

    #[test]
    fn diagram_binary_fmt() {
        let test = EngineDiagram("....#..25...1.*....@".to_string());
        assert_eq!(format!("{test:b}"), "00001000000000100001".to_string())
    }

    #[test]
    fn rotated_or_test() {
        assert_eq!(
            rotated_or_str(&"00001000000000100001".to_string(), 1),
            "10011100000001110011".to_string()
        );
        assert_eq!(
            rotated_or_str(&"00001000000000100001".to_string(), 5),
            "00001000010000100001".to_string()
        );
        assert_eq!(
            rotated_or_str(&"000010000".to_string(), 3),
            "010010010".to_string()
        );
    }
}
