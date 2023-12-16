#[derive(Clone, Debug)]
struct Field {
    visited: bool,
    steps: Vec<u64>,
    pipe: PipeDirection,
}

impl Field {
    fn new(direction: PipeDirection) -> Field {
        Field {
            visited: false,
            steps: Vec::new(),
            pipe: direction,
        }
    }

    fn visit(&mut self, step_count: u64) {
        self.visited = true;
        self.steps.push(step_count);
    }

    fn can_i_visit(&self, direction: Direction) -> bool {
        if self.visited || self.pipe == PipeDirection::Ground || self.pipe == PipeDirection::Start {
            return false;
        }

        match (direction, &self.pipe) {
            (Direction::North, PipeDirection::NorthSouth) => true,
            (Direction::North, PipeDirection::SouthEast) => true,
            (Direction::North, PipeDirection::SouthWest) => true,
            (Direction::East, PipeDirection::EastWest) => true,
            (Direction::East, PipeDirection::NorthWest) => true,
            (Direction::East, PipeDirection::SouthWest) => true,
            (Direction::South, PipeDirection::NorthSouth) => true,
            (Direction::South, PipeDirection::NorthEast) => true,
            (Direction::South, PipeDirection::NorthWest) => true,
            (Direction::West, PipeDirection::EastWest) => true,
            (Direction::West, PipeDirection::SouthEast) => true,
            (Direction::West, PipeDirection::NorthEast) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum PipeDirection {
    SouthEast,
    SouthWest,
    EastWest,
    NorthEast,
    NorthWest,
    NorthSouth,
    Ground,
    Start,
}

impl PipeDirection {
    fn next_direction(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (PipeDirection::NorthSouth, Direction::North) => Some(Direction::North),
            (PipeDirection::NorthSouth, Direction::South) => Some(Direction::South),
            (PipeDirection::NorthEast, Direction::South) => Some(Direction::East),
            (PipeDirection::NorthEast, Direction::West) => Some(Direction::North),
            (PipeDirection::NorthWest, Direction::South) => Some(Direction::West),
            (PipeDirection::NorthWest, Direction::East) => Some(Direction::North),
            (PipeDirection::SouthEast, Direction::North) => Some(Direction::East),
            (PipeDirection::SouthEast, Direction::West) => Some(Direction::South),
            (PipeDirection::SouthWest, Direction::North) => Some(Direction::West),
            (PipeDirection::SouthWest, Direction::East) => Some(Direction::South),
            (PipeDirection::EastWest, Direction::East) => Some(Direction::East),
            (PipeDirection::EastWest, Direction::West) => Some(Direction::West),
            (PipeDirection::Ground, _) => None,
            (PipeDirection::Start, _) => None,
            (_, _) => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for PipeDirection {
    fn from(c: char) -> PipeDirection {
        match c {
            'F' => PipeDirection::SouthEast,
            '7' => PipeDirection::SouthWest,
            '-' => PipeDirection::EastWest,
            'L' => PipeDirection::NorthEast,
            'J' => PipeDirection::NorthWest,
            '|' => PipeDirection::NorthSouth,
            '.' => PipeDirection::Ground,
            'S' => PipeDirection::Start,
            _ => panic!("invalid direction"),
        }
    }
}

impl From<char> for Field {
    fn from(c: char) -> Field {
        Field::new(PipeDirection::from(c))
    }
}

fn main() {
    let input = include_str!("assets/day10/input");
    println!("part 1: {}", solve_part1(input));

    //println!("part 2: {}", solve_part2(input));
}

fn walk_maze(mut map: Vec<Vec<Field>>) -> u64 {
    let (start_y, row) = map
        .iter()
        .enumerate()
        .find(|(_, row)| row.iter().any(|field| field.pipe == PipeDirection::Start))
        .unwrap();

    let start_x = row
        .iter()
        .enumerate()
        .find(|(_, field)| field.pipe == PipeDirection::Start)
        .unwrap()
        .0;

    // identify all possible paths from start
    let mut paths: Vec<(usize, usize, Direction)> = vec![];

    // first step go right
    let right_neighbor = if start_x == map.len() - 1 {
        None
    } else {
        Some(&mut map[start_y][start_x + 1])
    };

    if let Some(right_neighbor) = right_neighbor {
        if right_neighbor.can_i_visit(Direction::East) {
            paths.push((start_x + 1, start_y, Direction::East));
            right_neighbor.visit(1)
        }
    }

    // second go up
    let top_neighbor = if start_y == 0 {
        None
    } else {
        Some(&mut map[start_y - 1][start_x])
    };

    if let Some(top_neighbor) = top_neighbor {
        if top_neighbor.can_i_visit(Direction::North) {
            paths.push((start_x, start_y - 1, Direction::North));
            top_neighbor.visit(1)
        }
    }

    // third go left
    let left_neighbor = if start_x == 0 {
        None
    } else {
        Some(&mut map[start_y][start_x - 1])
    };

    if let Some(left_neighbor) = left_neighbor {
        if left_neighbor.can_i_visit(Direction::West) {
            paths.push((start_x - 1, start_y, Direction::West));
            left_neighbor.visit(1)
        }
    }

    // fourth go down
    let bottom_neighbor = if start_y == map.len() - 1 {
        None
    } else {
        Some(&mut map[start_y + 1][start_x])
    };

    if let Some(bottom_neighbor) = bottom_neighbor {
        if bottom_neighbor.can_i_visit(Direction::South) {
            paths.push((start_x, start_y + 1, Direction::South));
            bottom_neighbor.visit(1)
        }
    }

    for (path_x, path_y, direction) in paths {
        let mut current_y = path_y;
        let mut current_x = path_x;
        let mut current_direction = direction.clone();

        // already one step for the start
        let mut steps = 1u64;

        loop {
            // get current field
            let current_field = &mut map[current_y][current_x];

            // get next possible direction
            let next_possible_direction = current_field.pipe.next_direction(current_direction.clone());

            // if no next possible direction: end of the path
            let Some(next_possible_direction) = next_possible_direction else {
                // clear visited for next path
                map.iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|field| field.visited = false));
                break;
            };

            // get next field
            let next_field = match next_possible_direction {
                Direction::North => {
                    if current_y == 0 {
                        break;
                    }
                    &mut map[current_y - 1][current_x]
                }
                Direction::South => {
                    if current_y == map.len() - 1 {
                        break;
                    }
                    &mut map[current_y + 1][current_x]
                }
                Direction::East => {
                    if current_x == map.len() - 1 {
                        break;
                    }
                    &mut map[current_y][current_x + 1]
                }
                Direction::West => {
                    if current_x == 0 {
                        break;
                    }
                    &mut map[current_y][current_x - 1]
                }
            };

            // if next field can be visited - visit it
            if next_field.can_i_visit(next_possible_direction.clone()) {
                steps += 1;
                next_field.visit(steps);
                match next_possible_direction {
                    Direction::North => current_y -= 1,
                    Direction::South => current_y += 1,
                    Direction::East => current_x += 1,
                    Direction::West => current_x -= 1,
                }
                current_direction = next_possible_direction;
            } else {
                // clear visited for next path
                map.iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|field| field.visited = false));
                break;
            }
        }
    }

    *map.iter()
        .flatten()
        .map(|field| field.steps.iter().min().unwrap_or(&0))
        .max()
        .unwrap()
}

fn solve_part1(input: &str) -> u64 {
    let map = input
        .lines()
        .map(|line| line.chars().map(Field::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    walk_maze(map)
}

fn solve_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_parses_the_map_correctly() {
        let input = include_str!("assets/day10/input_test1");

        let map = input
            .lines()
            .map(|line| line.chars().map(Field::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(map[1][1].pipe, PipeDirection::Start);
        assert_eq!(map[1][2].pipe, PipeDirection::EastWest);
        assert_eq!(map[1][3].pipe, PipeDirection::SouthWest);

        assert_eq!(map[2][1].pipe, PipeDirection::NorthSouth);
        assert_eq!(map[2][2].pipe, PipeDirection::Ground);
        assert_eq!(map[2][3].pipe, PipeDirection::NorthSouth);

        assert_eq!(map[3][1].pipe, PipeDirection::NorthEast);
        assert_eq!(map[3][2].pipe, PipeDirection::EastWest);
        assert_eq!(map[3][3].pipe, PipeDirection::NorthWest);

        assert!(map[0]
            .iter()
            .all(|field| field.pipe == PipeDirection::Ground));
        assert!(map[4]
            .iter()
            .all(|field| field.pipe == PipeDirection::Ground));
        assert_eq!(map[0][4].pipe, PipeDirection::Ground);
        assert_eq!(map[2][4].pipe, PipeDirection::Ground);
        assert_eq!(map[3][4].pipe, PipeDirection::Ground);
        assert_eq!(map[4][4].pipe, PipeDirection::Ground);
        assert_eq!(map[0][0].pipe, PipeDirection::Ground);
        assert_eq!(map[2][0].pipe, PipeDirection::Ground);
        assert_eq!(map[3][0].pipe, PipeDirection::Ground);
        assert_eq!(map[4][0].pipe, PipeDirection::Ground);
    }

    #[test]
    fn test_part1_test1() {
        let input = include_str!("assets/day10/input_test1");
        assert_eq!(solve_part1(input), 4);
    }

    #[test]
    fn test_part1_test2() {
        let input = include_str!("assets/day10/input_test2");
        assert_eq!(solve_part1(input), 8);
    }

}
