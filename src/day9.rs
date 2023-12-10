fn main() {
    let input = include_str!("assets/day9/input");

    let result = solve_part1(input);
    println!("Part 1: {}", result);

    let result = solve_part2(input);
    println!("Part 2: {}", result);

}

fn solve_part1(input: &str) -> i64 {
    let measures = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|val| !val.is_empty())
                .flat_map(|val| val.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    let mut differences = Vec::new();

    for measure in &measures {
        let mut measure = measure.clone();
        let mut diffs = Vec::new();

        loop {
            let diff = measure
                .iter()
                .enumerate()
                .flat_map(|(idx, num)| {
                    if idx == measure.len() - 1 {
                        None
                    } else {
                        Some(measure[idx + 1] - num)
                    }
                })
                .collect::<Vec<i64>>();

            if diff.iter().all(|num| *num == 0) {
                break;
            }

            measure = diff.clone();

            diffs.push(diff);
        }

        differences.push(diffs);
    }

    let mut new_diffs = Vec::new();

    for mut measure_diffs in differences {
        let len = measure_diffs.len();
        for idx in (0..len).rev() {
            let val = if idx == len - 1 {
                *measure_diffs[idx].last().unwrap()
            } else {
                *measure_diffs[idx].last().unwrap() + *measure_diffs[idx + 1].last().unwrap()
            };

            measure_diffs[idx].push(val);
        }

        new_diffs.push(measure_diffs);
    }

    measures
        .iter()
        .zip(new_diffs.iter())
        .map(|(measure, diffs)| {
            measure.last().unwrap() + diffs.first().unwrap().last().unwrap()
        })
        .sum()
}

fn solve_part2(input: &str) -> i64 {
    let measures = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|val| !val.is_empty())
                .flat_map(|val| val.parse::<i64>())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    let mut differences = Vec::new();

    for measure in &measures {
        let mut measure = measure.clone();
        let mut diffs = Vec::new();

        loop {
            let diff = measure
                .iter()
                .enumerate()
                .flat_map(|(idx, num)| {
                    if idx == measure.len() - 1 {
                        None
                    } else {
                        Some(measure[idx + 1] - num)
                    }
                })
                .collect::<Vec<i64>>();

            if diff.iter().all(|num| *num == 0) {
                break;
            }

            measure = diff.clone();

            diffs.push(diff);
        }

        differences.push(diffs);
    }

    let mut new_diffs = Vec::new();

    for mut measure_diffs in differences {
        let len = measure_diffs.len();
        for idx in (0..len).rev() {
            let val = if idx == len - 1 {
                *measure_diffs[idx].last().unwrap()
            } else {
                *measure_diffs[idx].first().unwrap() - *measure_diffs[idx + 1].first().unwrap()
            };

            measure_diffs[idx].insert(0, val);
        }

        new_diffs.push(measure_diffs);
    }

    measures
        .iter()
        .zip(new_diffs.iter())
        .map(|(measure, diffs)| {
            measure.first().unwrap() - diffs.first().unwrap().first().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("assets/day9/input_test");
        let result = solve_part1(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_works() {
        let input = include_str!("assets/day9/input_test");
        let result = solve_part2(input);
        assert_eq!(result, 2);

    }
}
