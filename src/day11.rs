use std::{collections::HashSet, fmt::Formatter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Galaxy(usize, usize);

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        // distance in 2D matrix
        (self.0 as isize - other.0 as isize).unsigned_abs()
            + (self.1 as isize - other.1 as isize).unsigned_abs()
    }

    fn shift_x(&mut self, x_shift_indices: &[usize], scale: Option<usize>) {
        let scale = scale.unwrap_or(1);

        let shift = x_shift_indices
            .iter()
            .filter(|&&shift_index| shift_index < self.0)
            .map(|_| if scale == 1 { 1 } else { scale - 1 })
            .sum::<usize>();

        self.0 = (self.0 as isize + shift as isize) as usize;
    }

    fn shift_y(&mut self, y_shift_indices: &[usize], scale: Option<usize>) {
        let scale = scale.unwrap_or(1);

        let shift = y_shift_indices
            .iter()
            .filter(|&&shift_index| shift_index < self.1)
            .map(|_| if scale == 1 { 1 } else { scale - 1 })
            .sum::<usize>();

        self.1 = (self.1 as isize + shift as isize) as usize;
    }
}

#[derive(PartialEq, Clone)]
enum SpaceField {
    Space,
    Galaxy,
}

impl SpaceField {
    fn is_galaxy(&self) -> bool {
        matches!(self, SpaceField::Galaxy)
    }
}

impl std::fmt::Debug for SpaceField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SpaceField::Space => write!(f, "."),
            SpaceField::Galaxy => write!(f, "#"),
        }
    }
}

impl From<char> for SpaceField {
    fn from(c: char) -> Self {
        match c {
            '.' => SpaceField::Space,
            '#' => SpaceField::Galaxy,
            _ => panic!("Unknown space field: {}", c),
        }
    }
}

fn solve_part1(input: &str) -> usize {
    solve_by_shift(input, 1)
}

fn solve_part_2(input: &str) -> usize {
    solve_by_shift(input, 1_000_000)
}

fn solve_by_shift(input: &str, scale: usize) -> usize {
    let space = parse_input(input);

    let mut galaxies = get_galaxies(&space);

    let (empty_rows, empty_cols) = get_empty_space(&space);

    galaxies.iter_mut().for_each(|galaxy| {
        galaxy.shift_x(&empty_cols, Some(scale));
        galaxy.shift_y(&empty_rows, Some(scale));
    });

    let galaxy_pairs = get_galaxy_pairs(&galaxies);

    galaxy_pairs
        .iter()
        .map(|(g1, g2)| g1.distance(g2))
        .sum::<usize>()
}

fn get_galaxies(space: &[Vec<SpaceField>]) -> Vec<Galaxy> {
    space
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, field)| field.is_galaxy())
                .map(|(x, _)| Galaxy(x, y))
                .collect::<Vec<Galaxy>>()
        })
        .collect::<Vec<Galaxy>>()
}

fn get_galaxy_pairs(galaxies: &[Galaxy]) -> HashSet<(Galaxy, Galaxy)> {
    let mut pairs = HashSet::new();

    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            pairs.insert((galaxy.clone(), other_galaxy.clone()));
        }
    }

    pairs
}

fn get_empty_space(space: &[Vec<SpaceField>]) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = space
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|field| field == &SpaceField::Space))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    // get values of a column
    let mapped_col_vals = (0..space[0].len())
        .map(|i| {
            space
                .iter()
                .map(|row| row[i].clone())
                .collect::<Vec<SpaceField>>()
        })
        .collect::<Vec<_>>();

    // find empty columns
    let empty_cols = mapped_col_vals
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|field| field == &SpaceField::Space))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    (empty_rows, empty_cols)
}

fn parse_input(input: &str) -> Vec<Vec<SpaceField>> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .map(SpaceField::from)
                .collect::<Vec<SpaceField>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("assets/day11/input");
    let result = solve_part1(input);
    println!("Part 1: {}", result);

    let result = solve_part_2(input);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("assets/day11/input_test");
        let space = parse_input(input);

        assert_eq!(space[0][3], SpaceField::Galaxy);
        assert_eq!(space[1][7], SpaceField::Galaxy);
        assert_eq!(space[2][0], SpaceField::Galaxy);
        assert_eq!(space[4][6], SpaceField::Galaxy);
        assert_eq!(space[5][1], SpaceField::Galaxy);
        assert_eq!(space[6][9], SpaceField::Galaxy);
        assert_eq!(space[8][7], SpaceField::Galaxy);
        assert_eq!(space[9][0], SpaceField::Galaxy);
        assert_eq!(space[9][4], SpaceField::Galaxy);

        assert!(space[3].iter().all(|field| field == &SpaceField::Space));
        assert!(space[7].iter().all(|field| field == &SpaceField::Space));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("assets/day11/input_test");
        let sum = solve_part1(input);
        assert_eq!(sum, 374);
    }

    #[test]
    fn test_part1_by_shifting() {
        let input = include_str!("assets/day11/input_test");
        let sum = solve_by_shift(input, 1);
        assert_eq!(sum, 374);
    }

    #[test]
    fn test_solving_with_10x_expand() {
        let input = include_str!("assets/day11/input_test");
        let sum = solve_by_shift(input, 10);
        assert_eq!(sum, 1030);
    }

    #[test]
    fn test_solving_with_100x_expand() {
        let input = include_str!("assets/day11/input_test");
        let sum = solve_by_shift(input, 100);
        assert_eq!(sum, 8410);
    }
}
