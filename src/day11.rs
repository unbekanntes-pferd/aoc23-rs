use std::{collections::HashSet, fmt::Formatter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Galaxy(usize, usize);

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        // distance in 2D matrix 
        (self.0 as isize - other.0 as isize).abs() as usize
            + (self.1 as isize - other.1 as isize).abs() as usize
    }
}

#[derive(PartialEq, Clone)]
enum SpaceField {
    Space,
    Galaxy,
}

impl SpaceField {
    fn is_galaxy(&self) -> bool {
        match self {
            SpaceField::Galaxy => true,
            _ => false,
        }
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
    let mut space = parse_input(input);
    let expanded_space = expand_space(&mut space);

    let galaxies = get_galaxies(&expanded_space);
    let galaxy_pairs = get_galaxy_pairs(&galaxies);

    galaxy_pairs
        .iter()
        .map(|(g1, g2)| g1.distance(g2))
        .sum::<usize>()
}

fn get_galaxies(space: &Vec<Vec<SpaceField>>) -> Vec<Galaxy> {
    space
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, field)| field.is_galaxy())
                .map(|(x, _)| Galaxy(x, y))
                .collect::<Vec<Galaxy>>()
        })
        .flatten()
        .collect::<Vec<Galaxy>>()
}

fn get_galaxy_pairs(galaxies: &Vec<Galaxy>) -> HashSet<(Galaxy, Galaxy)> {
    let mut pairs = HashSet::new();

    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            pairs.insert((galaxy.clone(), other_galaxy.clone()));
        }
    }

    pairs
}

fn parse_input(input: &str) -> Vec<Vec<SpaceField>> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(SpaceField::from)
                .collect::<Vec<SpaceField>>()
        })
        .collect::<Vec<_>>()
}

fn expand_space(space: &mut Vec<Vec<SpaceField>>) -> Vec<Vec<SpaceField>> {
    // first find empty rows
    let empty_rows = space
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|field| field == &SpaceField::Space))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let empty_row = vec![SpaceField::Space; space[0].len()];

    // duplicate empty rows
    for (count, row_idx) in empty_rows.iter().enumerate() {
        space.insert(row_idx + count, empty_row.clone());
    }

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

    // duplicate empty columns
    for (count, col_idx) in empty_cols.iter().enumerate() {
        for row in space.iter_mut() {
            row.insert(col_idx + count, SpaceField::Space);
        }
    }

    space.clone()
}

fn main() {
    let input = include_str!("assets/day11/input");
    solve_part1(input);
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
    fn test_expand_space() {
        let input = include_str!("assets/day11/input_test");
        let space = parse_input(input);

        let expanded_space = expand_space(&mut space.clone());
        println!("{:?}", expanded_space);
        assert!(expanded_space[3]
            .iter()
            .all(|field| field == &SpaceField::Space));
        assert!(expanded_space[4]
            .iter()
            .all(|field| field == &SpaceField::Space));
        assert!(expanded_space[8]
            .iter()
            .all(|field| field == &SpaceField::Space));
        assert!(expanded_space[9]
            .iter()
            .all(|field| field == &SpaceField::Space));

        expanded_space.iter().for_each(|row| {
            assert_eq!(row.len(), 13);
        });

        assert_eq!(expanded_space[0][4], SpaceField::Galaxy);
        assert_eq!(expanded_space[1][9], SpaceField::Galaxy);
        assert_eq!(expanded_space[2][0], SpaceField::Galaxy);
        assert_eq!(expanded_space[5][8], SpaceField::Galaxy);
        assert_eq!(expanded_space[6][1], SpaceField::Galaxy);
        assert_eq!(expanded_space[7][12], SpaceField::Galaxy);
        assert_eq!(expanded_space[10][9], SpaceField::Galaxy);
        assert_eq!(expanded_space[11][0], SpaceField::Galaxy);
        assert_eq!(expanded_space[11][5], SpaceField::Galaxy);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("assets/day11/input_test");
        let sum = solve_part1(input);

        assert_eq!(sum, 374);
    }
}
