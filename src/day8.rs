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

    let result = solve_part2(input);
    println!("Part 2: {}", result);
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

fn solve_part2(input: &str) -> usize {
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

    let start_locations = locations
        .keys()
        .filter(|loc| loc.ends_with('A'))
        .map(|loc| loc.to_owned())
        .collect::<Vec<_>>();

    let mut iterations = Vec::new();

    for start_location in start_locations {
        let mut location = start_location;
        let mut step_count = 0u64;
        let mut directions_iter = directions.iter().cycle();

        while !location.ends_with('Z') {
            if let Some(direction) = directions_iter.next() {
                location = match direction {
                    Direction::Left => locations[&location].0.clone(),
                    Direction::Right => locations[&location].1.clone(),
                };
                step_count += 1;
            }
        }

        iterations.push(step_count);
    }

    let mut least_common_multiple = iterations.pop().unwrap();

    for iteration in iterations {
        least_common_multiple =
            least_common_multiple * iteration / gcd::binary_u64(least_common_multiple, iteration)
    }

    least_common_multiple as usize
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

    #[test]
    fn part2_test1_works() {
        let input = include_str!("assets/day8/input_test3");
        let result = solve_part2(input);
        assert_eq!(result, 6);
    }
}
