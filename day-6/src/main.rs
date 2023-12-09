#[derive(Debug)]
struct RaceRecord {
    time_allowed: f64,
    max_distance: f64,
}

fn main() {
    let input = include_str!("../input.txt");
    let race_records = parse_races(input);

    let answer_one = problem_one(race_records);
    let answer_two = problem_two(&input);

    println!("There are {answer_one} ways to win if you read the slip as separate races.");
    println!("There are {answer_two} ways to win if you read the slip as one mega race.");
}

fn problem_two(input: &str) -> i32 {
    let mega_race = parse_races_ignore_whitespace(input);

    let intersections = quadratic_formula(-1_f64, mega_race.time_allowed, -mega_race.max_distance);
    let winning_bounds = round_bounds(intersections.0, intersections.1);
    
    winning_bounds.1 - winning_bounds.0 + 1
}

fn problem_one(race_records: Vec<RaceRecord>) -> i32 {
    race_records
        .iter()
        .enumerate()
        .for_each(|(i, r)| println!("{i}: {:?}", r));

    // We need to solve the solve for the system of equations:
    // y < time_allowed
    // d > max_distance
    // and y = x(d - x).
    // solving for x.

    // Using substitution, we can arrange the equation to be
    // -x^2 + time_allowed * x - max_distance = 0

    // The two points where this equation crosses the y-axis
    // represents the lower and upper bounds of times holding the button
    // that will beat the current record.
    let intersections = race_records
        .iter()
        .map(|r| quadratic_formula(-1_f64, r.time_allowed, -r.max_distance));

    let winning_bounds = intersections.map(|(l, u)| round_bounds(l, u));

    winning_bounds.fold(1, |prev, (l, u)| (u - l + 1) * prev)
}

fn round_bounds(lower_bound: f64, upper_bound: f64) -> (i32, i32) {
    let lb = match lower_bound.ceil() == lower_bound {
        true => lower_bound + 1_f64,
        false => lower_bound.ceil(),
    };
    let ub = match upper_bound.floor() == upper_bound {
        true => upper_bound - 1_f64,
        false => upper_bound.floor(),
    };

    (lb as i32, ub as i32)
}

fn parse_races(input: &str) -> Vec<RaceRecord> {
    let mut lines = input.lines();
    let times_str = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace();
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace();

    times_str
        .zip(distances)
        .map(|(t_str, d_str)| {
            let t: f64 = t_str.parse().unwrap();
            let d: f64 = d_str.parse().unwrap();

            RaceRecord {
                time_allowed: t,
                max_distance: d,
            }
        })
        .collect()
}

fn parse_races_ignore_whitespace(input: &str) -> RaceRecord {
    let mut lines = input.lines();
    let times_str = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .fold(String::from(""), |mut prev, curr| {
            prev.push_str(curr);
            return prev;
        });
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .fold(String::from(""), |mut prev, curr| {
            prev.push_str(curr);
            return prev;
        });

    RaceRecord {
        time_allowed: times_str.parse().unwrap(),
        max_distance: distances.parse().unwrap(),
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sqrt = (b.powf(2_f64) - 4_f64 * a * c).sqrt();

    ((-b + sqrt) / 2_f64 * a, (-b - sqrt) / 2_f64 * a)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_EPSILON: f64 = 0.0000001;

    #[test]
    fn quadratic() {
        let q = quadratic_formula(-1_f64, 7_f64, -9_f64);
        println!("{:?}", q);
        assert!((q.0 - 1.6972244).abs() < TEST_EPSILON);
        assert!((q.1 - 5.3027754).abs() < TEST_EPSILON);
    }
}
