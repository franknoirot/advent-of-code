#[derive(Debug, PartialEq)]
struct MapLine {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut sections = input.split_terminator("\n\n");
    let seeds_flat = sections
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let parsed_sections = sections.map(parse_section);
    let section_maps: Vec<_> = parsed_sections.map(build_section_fn).collect();

    let destinations = seeds_flat
        .chunks(2)
        .flat_map(|chunk| {
            let seed_start = chunk[0];
            let seed_offset = chunk[1];
            let seed_range = seed_start..(seed_start + seed_offset);

            seed_range.map(|s| section_maps.iter().fold(s, |input, func| func(input)))
        })
        .min();

    println!("The nearest destination is {:?}", destinations.unwrap());
}

fn build_section_fn(sections: Vec<MapLine>) -> Box<impl Fn(i64) -> i64 + 'static> {
    Box::new(move |input| {
        for section in &sections {
            if input >= section.src_start && input < section.src_start + section.length {
                return section.dest_start + input - section.src_start;
            }
        }

        return input;
    })
}

fn parse_section(section: &str) -> Vec<MapLine> {
    let mut lines = section.lines();
    lines.next(); // We can call next to ignore the first (labeling) line

    lines.map(parse_map_line).collect()
}

fn parse_map_line(line: &str) -> MapLine {
    let nums: Vec<i64> = line
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    MapLine {
        dest_start: nums[0],
        src_start: nums[1],
        length: nums[2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        let test = "50 98 2";

        assert_eq!(
            parse_map_line(test),
            MapLine {
                dest_start: 50,
                src_start: 98,
                length: 2,
            }
        );

        let test_mid_zero = "3378130613 0 34101494";
        assert_eq!(
            parse_map_line(test_mid_zero),
            MapLine {
                dest_start: 3378130613,
                src_start: 0,
                length: 34101494,
            }
        )
    }

    #[test]
    fn section_parsing() {
        let test = "seed-to-soil map:\n50 98 2\n52 50 48\n";

        assert_eq!(
            parse_section(test),
            vec![
                MapLine {
                    dest_start: 50,
                    src_start: 98,
                    length: 2,
                },
                MapLine {
                    dest_start: 52,
                    src_start: 50,
                    length: 48,
                },
            ]
        );
    }

    #[test]
    fn section_fn_building() {
        let test = vec![
            MapLine {
                dest_start: 50,
                src_start: 98,
                length: 2,
            },
            MapLine {
                dest_start: 52,
                src_start: 50,
                length: 48,
            },
        ];

        let test_fn = build_section_fn(test);

        assert_eq!(test_fn(79), 81);
        assert_eq!(test_fn(14), 14);
        assert_eq!(test_fn(55), 57);
        assert_eq!(test_fn(13), 13);
    }
}
