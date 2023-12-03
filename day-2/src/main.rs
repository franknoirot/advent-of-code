#[derive(PartialEq, Debug)]
enum Draw {
    Red(i32),
    Green(i32),
    Blue(i32),
}
type Id = i32;
type Game = (Id, Vec<Draw>);

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let input = include_str!("../input.txt");

    let parsed_games = input.lines().map(parse_game);

    let possible_games = parsed_games
        .clone()
        .filter(|(_, draws)| draws.into_iter().all(draw_is_possible));
    let summed_ids = possible_games.map(|(index, _)| index).sum::<i32>();
    println!("Problem 1: The sum of the ID's of the possible games is {summed_ids}");

    let summed_powers: i32 = parsed_games
        .map(get_game_power)
        .map(|(r, g, b)| r * g * b)
        .sum();

    println!("Problem 2: The sum of the minimum powers for all games is {summed_powers}");
}

fn get_game_power(game: Game) -> (i32, i32, i32) {
    let (_, draws) = game;
    let mut maxes = (0, 0, 0);

    draws.iter().for_each(|d| match d {
        Draw::Red(count) => maxes.0 = *count.max(&maxes.0),
        Draw::Green(count) => maxes.1 = *count.max(&maxes.1),
        Draw::Blue(count) => maxes.2 = *count.max(&maxes.2),
    });

    maxes
}

fn draw_is_possible(draw: &Draw) -> bool {
    match draw {
        Draw::Red(count) => count <= &MAX_RED,
        Draw::Green(count) => count <= &MAX_GREEN,
        Draw::Blue(count) => count <= &MAX_BLUE,
    }
}

fn parse_game(game_str: &str) -> Game {
    let mut game_split = game_str.split(':');
    let id = game_split
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let draws = game_split.last().unwrap().split(';');

    (
        id,
        draws
            .flat_map(|d| d.split(',').map(parse_draw).collect::<Vec<Draw>>())
            .collect(),
    )
}

fn parse_draw(cube: &str) -> Draw {
    let mut results = cube.split_whitespace();
    let count = results.next().unwrap().parse::<i32>().unwrap();
    let label = results.last().unwrap().trim();

    match label {
        "red" => Draw::Red(count),
        "green" => Draw::Green(count),
        "blue" => Draw::Blue(count),
        _ => panic!("there is a cube's text that has a non-standard color!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_parsing() {
        assert_eq!(parse_draw("14 red"), Draw::Red(14));
        assert_eq!(parse_draw("17 green"), Draw::Green(17));
        assert_eq!(parse_draw("1 blue"), Draw::Blue(1));
    }

    #[test]
    #[should_panic]
    fn cube_failing() {
        parse_draw("14 vermillion");
    }

    #[test]
    fn get_power() {
        use crate::Draw::{Blue, Green, Red};
        // Game 100: 1 blue, 13 green, 14 red; 11 green, 11 blue, 7 red; 2 red, 1 blue, 2 green; 10 blue, 15 red
        let game: Game = (
            100,
            vec![
                Blue(1),
                Green(13), // highest green
                Red(14),
                Green(11),
                Blue(11), // highest blue
                Red(7),
                Red(2),
                Blue(1),
                Green(2),
                Blue(10),
                Red(15), // highest red
            ],
        );
        assert_eq!(get_game_power(game), (15, 13, 11));
    }
}
