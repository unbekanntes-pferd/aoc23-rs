enum Field {
    SouthEast,
    SouthWest,
    EastWest,
    NorthEast,
    NorthWest,
    NorthSouth,
    Ground,
    Start
}

impl From<char> for Field {
    fn from(c: char) -> Field {
        match c {
            'F' => Field::SouthEast,
            '7' => Field::SouthWest,
            '-' => Field::EastWest,
            'L' => Field::NorthEast,
            'J' => Field::NorthWest,
            '|' => Field::NorthSouth,
            '.' => Field::Ground,
            'S' => Field::Start,
            _ => panic!("invalid field"),
        }
    }
}


