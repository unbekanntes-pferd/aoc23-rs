use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}
fn main() {
    let input = include_str!("assets/day8/input");

    let result = solve_part1(input);
    println!("Part 1: {}", result);
}

fn solve_part1(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let directions = input
        .first()
        .unwrap()
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>();

    let locations = input
        .get(1)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let parts = s.split('=').collect::<Vec<_>>();
            let location = parts.first().unwrap().trim().to_string();

            let opt_parts = parts
                .get(1)
                .unwrap()
                .split(',')
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            let opt_parts = opt_parts
                .iter()
                .map(|s| s.trim_start_matches('(').trim_end_matches(')'))
                .collect::<Vec<_>>();

            assert_eq!(opt_parts.len(), 2);

            let options = (
                opt_parts.first().unwrap().to_string(),
                opt_parts.last().unwrap().to_string(),
            );

            (location, options)
        })
        .collect::<BTreeMap<_, _>>();

    let mut location = locations.keys().next().unwrap().to_owned();
    let mut count_steps = 0u64;

    loop {
        for direction in directions.clone() {
            location = match direction {
                Direction::Left => {
                    let location = &locations.get(&location).unwrap().0;
                    count_steps += 1;

                    location.to_owned()
                }
                Direction::Right => {
                    let location = &locations.get(&location).unwrap().1;
                    count_steps += 1;

                    location.to_owned()
                }
            }
        }

        if location == *locations.keys().last().unwrap() {
            break;
        }
    }

    count_steps as usize
}
fn solve_part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test1_works() {
        let input = include_str!("assets/day8/input_test1");
        let result = solve_part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_test2_works() {
        let input = include_str!("assets/day8/input_test2");
        let result = solve_part1(input);
        assert_eq!(result, 6);
    }
}
