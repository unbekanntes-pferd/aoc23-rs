#[derive(Debug, Clone)]
struct EngineRow {
    cells: Vec<EngineCell>,
}

impl EngineRow {
    fn new(cells: Vec<EngineCell>) -> Self {
        Self { cells }
    }
}

#[derive(Debug, Clone)]
struct EngineCell {
    value: Cell,
    index: usize,
    row: usize,
}

impl EngineCell {
    fn new(value: Cell, index: usize, row: usize) -> Self {
        Self { value, index, row }
    }

    fn is_symbol(&self) -> bool {
        match self.value {
            Cell::Symbol => true,
            _ => false,
        }
    }

    fn is_orphan(
        &self,
        own_row: EngineRow,
        upper_row: Option<EngineRow>,
        lower_row: Option<EngineRow>,
    ) -> bool {
        match self.value {
            Cell::Value(v) => {
                eprintln!("Looking at val: {}", v);

                let start_index = self.index;
                let end_index = self.index + self.value.len() - 1; // Last index occupied by the numeric segment

 
                let has_upper_neighbors = upper_row
                    .map(|row| {
                        row.cells.iter().any(|cell| {
                            cell.is_symbol()
                                && (cell.index >= start_index.saturating_sub(1)
                                    && cell.index <= end_index + 1)
                        })
                    })
                    .unwrap_or(false);


                let has_lower_neighbors = lower_row
                    .map(|row| {
                        row.cells.iter().any(|cell| {
                            cell.is_symbol()
                                && (cell.index >= start_index.saturating_sub(1)
                                    && cell.index <= end_index + 1)
                        })
                    })
                    .unwrap_or(false);

                // Check for same row neighbors
                let has_neighbors = own_row.cells.iter().any(|cell| {
                    cell.is_symbol()
                        && ((start_index > 0 && cell.index == start_index - 1)
                            || cell.index == end_index + 1)
                });

                !has_neighbors && !has_upper_neighbors && !has_lower_neighbors
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Cell {
    Empty,
    Symbol,
    Value(u16),
}

impl Cell {
    fn len(&self) -> usize {
        match self {
            Cell::Value(v) => v.to_string().len(),
            _ => 0,
        }
    }
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        eprintln!("parsing cell: {}", s);
        match s
            .chars()
            .next()
            .expect("slice must contain at least one char")
        {
            '.' => Cell::Empty,
            char if char.is_numeric() => Cell::Value(s.parse::<u16>().unwrap()),
            char if char.is_ascii_punctuation() => Cell::Symbol,
            _ => unreachable!("invalid format"),
        }
    }
}

trait IntoEngineRows {
    fn into_engine_rows(&self) -> Vec<EngineRow>;
}

impl IntoEngineRows for &str {
    fn into_engine_rows(&self) -> Vec<EngineRow> {
        self.lines()
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(row, s)| {
                let (_, mut segments) = s.chars().fold(
                    (String::new(), Vec::new()),
                    |(mut current_segment, mut segments), ch| {
                        match ch {
                            '.' | _ if !ch.is_alphanumeric() => {
                                if !current_segment.is_empty() {
                                    segments.push(current_segment.clone());
                                    current_segment.clear();
                                }
                                segments.push(ch.to_string());
                            }
                            _ => {
                                current_segment.push(ch);
                            }
                        };
                        (current_segment, segments)
                    },
                );

                if !segments.is_empty() && !segments.last().unwrap().is_empty() {
                    segments.push(segments.last().unwrap().clone());
                }

                let mut cells = Vec::new();
                let mut index = 0;

                for segment in segments {
                    if segment.chars().all(char::is_numeric) {
                        for _ in segment.chars() {
                            cells.push(EngineCell::new(Cell::from(segment.as_str()), index, row));
                            index += 1;
                        }
                    } else {
                        cells.push(EngineCell::new(Cell::from(segment.as_str()), index, row));
                        index += 1;
                    }
                }

                EngineRow::new(cells)
            })
            .collect::<Vec<EngineRow>>()
    }
}

fn main() {
    let input = include_str!("assets/day3/input");

    let engine_rows = input.into_engine_rows();

    let orphaned_numbers = engine_rows
        .iter()
        .map(|row| row.cells.clone())
        .flatten()
        .filter(|cell| {
            let own_row = engine_rows.get(cell.row).unwrap().clone();
            let upper_row = if cell.row > 0 {
                Some(engine_rows.get(cell.row - 1).unwrap().clone())
            } else {
                None
            };

            let lower_row = if cell.row < engine_rows.len() - 1 {
                Some(engine_rows.get(cell.row + 1).unwrap().clone())
            } else {
                None
            };

            cell.is_orphan(own_row, upper_row, lower_row)
        })
        .map(|cell| match cell.value {
            Cell::Value(v) => v,
            _ => unreachable!("should be value"),
        })
        .fold(std::collections::HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(num, count)| *count == num.to_string().len())
        .map(|(num, _)| num)
        .collect::<Vec<u16>>();

    let cell_numbers = engine_rows
        .iter()
        .map(|row| row.cells.clone())
        .flatten()
        .filter(|cell| match cell.value {
            Cell::Value(_) => true,
            _ => false,
        })
        .map(|cell| match cell.value {
            Cell::Value(v) => v,
            _ => unreachable!("should be value"),
        })
        .collect::<Vec<u16>>();

    let non_orphaned_numbers = cell_numbers
        .iter()
        .filter(|num| !orphaned_numbers.contains(num))
        .collect::<Vec<&u16>>();

    println!("non orphaned numbers: {:?}", non_orphaned_numbers);

    // remove duplicates
    let sum: u32 = non_orphaned_numbers.iter().map(|num| **num)
        .collect::<std::collections::HashSet<u16>>()
        .into_iter()
        .map(|num| num as u32)
        .sum();

    println!("sum: {}", sum);


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing_cell() {
        let cell = Cell::from(".");
        assert_eq!(cell, Cell::Empty);

        let cell = Cell::from("123");
        assert_eq!(cell, Cell::Value(123));

        let cell = Cell::from("!");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("$");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("/");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("*");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("@");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("=");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("&");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("$");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("+");
        assert_eq!(cell, Cell::Symbol);

        let cell = Cell::from("-");
        assert_eq!(cell, Cell::Symbol);
    }

    #[test]
    fn test_parsing_rows() {
        let rows = "...123......#34.\n12..76........!.";

        let engine_rows = rows.into_engine_rows();

        eprintln!("engine_rows: {:?}", engine_rows);

        assert_eq!(engine_rows.len(), 2);

        let first_row = engine_rows.get(0).unwrap();
        assert_eq!(first_row.cells.len(), 13);
        assert_eq!(first_row.cells.get(0).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(1).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(2).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(3).unwrap().value, Cell::Value(123));
        assert_eq!(first_row.cells.get(4).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(5).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(6).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(7).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(8).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(9).unwrap().value, Cell::Empty);
        assert_eq!(first_row.cells.get(10).unwrap().value, Cell::Symbol);
        assert_eq!(first_row.cells.get(11).unwrap().value, Cell::Value(34));
        assert_eq!(first_row.cells.get(6).unwrap().value, Cell::Empty);

        let second_row = engine_rows.get(1).unwrap();

        assert_eq!(second_row.cells.len(), 14);
        assert_eq!(second_row.cells.get(0).unwrap().value, Cell::Value(12));
        assert_eq!(second_row.cells.get(1).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(2).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(3).unwrap().value, Cell::Value(76));
        assert_eq!(second_row.cells.get(4).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(5).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(6).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(7).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(8).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(9).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(10).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(11).unwrap().value, Cell::Empty);
        assert_eq!(second_row.cells.get(12).unwrap().value, Cell::Symbol);
        assert_eq!(second_row.cells.get(13).unwrap().value, Cell::Empty);
    }

    #[test]
    fn it_sums_correctly() {
        let input = include_str!("assets/day3/input_test");

        let engine_rows = input.into_engine_rows();
    
        let orphaned_numbers = engine_rows
            .iter()
            .map(|row| row.cells.clone())
            .flatten()
            .filter(|cell| {
                let own_row = engine_rows.get(cell.row).unwrap().clone();
                let upper_row = if cell.row > 0 {
                    Some(engine_rows.get(cell.row - 1).unwrap().clone())
                } else {
                    None
                };
    
                let lower_row = if cell.row < engine_rows.len() - 1 {
                    Some(engine_rows.get(cell.row + 1).unwrap().clone())
                } else {
                    None
                };
    
                cell.is_orphan(own_row, upper_row, lower_row)
            })
            .map(|cell| match cell.value {
                Cell::Value(v) => v,
                _ => unreachable!("should be value"),
            })
            .fold(std::collections::HashMap::new(), |mut acc, num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter(|(num, count)| *count == num.to_string().len())
            .map(|(num, _)| num)
            .collect::<Vec<u16>>();
    
        let cell_numbers = engine_rows
            .iter()
            .map(|row| row.cells.clone())
            .flatten()
            .filter(|cell| match cell.value {
                Cell::Value(_) => true,
                _ => false,
            })
            .map(|cell| match cell.value {
                Cell::Value(v) => v,
                _ => unreachable!("should be value"),
            })
            .collect::<Vec<u16>>();
    
        let non_orphaned_numbers = cell_numbers
            .iter()
            .filter(|num| !orphaned_numbers.contains(num))
            .collect::<Vec<&u16>>();
    
    
        // remove duplicates with set
        let sum: u32 = non_orphaned_numbers.iter().map(|num| **num)
            .collect::<std::collections::HashSet<u16>>()
            .into_iter()
            .map(|num| num as u32)
            .sum();

        assert_eq!(sum, 4361);
    }
}
