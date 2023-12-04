#[derive(Debug)]
struct Engine {
    rows: Vec<EngineRow>,
}

impl From<Vec<EngineRow>> for Engine {
    fn from(rows: Vec<EngineRow>) -> Self {
        Self { rows }
    }
}

impl Engine {
    fn get_orphan_numbers() -> Vec<u16> {}

}

#[derive(Debug)]
struct EngineRow {
    cells: Vec<EngineCell>,
}

impl EngineRow {
    fn new(cells: Vec<EngineCell>) -> Self {
        Self { cells }
    }

    fn get_row_cells(&self, row_id: usize) -> Vec<EngineCell> {
        self.cells
            .iter()
            .filter(|cell| cell.row == row_id)
            .map(|cell| *cell.clone())
            .collect()
    }
}

#[derive(Debug)]
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
            Cell::Value(_) => true,
            _ => false,
        }
    }

    fn is_orphan(&self, others: &[EngineRow]) -> bool {
        match self.value {
            Cell::Value(v) => {
                let own_row: Vec<_> = others
                    .iter()
                    .filter(|rows| {
                        let cells: Vec<_> = rows
                            .cells
                            .iter()
                            .filter(|cell| cell.row == self.row)
                            .collect();
                        cells.len() > 0
                    })
                    .collect();

                let has_neighbors = own_row
                    .iter()
                    .find(|row| {
                        row.cells
                            .iter()
                            .find(|cell| {
                                cell.is_symbol()
                                    && (cell.index == self.index
                                        || (cell.index == self.index + self.value.len()))
                            })
                            .is_some()
                    })
                    .is_some();

                let has_upper_row_matches = if self.row > 0 {
                    let upper_row_cells = 
                }

                has_neighbors
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
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
        let rows = self
            .split("\n")
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(row, s)| {
                let (_, segments) = s.chars().fold(
                    (String::new(), Vec::new()),
                    |(mut current_segment, mut segments), ch| {
                        match ch {
                            '.' => {
                                if !current_segment.is_empty() {
                                    segments.push(current_segment.clone());
                                    current_segment.clear();
                                }
                                segments.push(".".to_string());
                            }
                            c if c.is_numeric() => {
                                if current_segment.chars().all(char::is_numeric) {
                                    current_segment.push(c);
                                } else {
                                    if !current_segment.is_empty() {
                                        segments.push(current_segment.clone());
                                        current_segment.clear();
                                    }
                                    current_segment.push(c);
                                }
                            }
                            _ if !ch.is_alphanumeric() => {
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

                let cells = if !segments.is_empty() {
                    segments
                        .into_iter()
                        .enumerate()
                        .map(|(index, val)| EngineCell::new(Cell::from(val.as_str()), index, row))
                        .collect::<Vec<EngineCell>>()
                } else {
                    Vec::new()
                };
                EngineRow::new(cells)
            })
            .collect::<Vec<EngineRow>>();

        rows
    }
}

fn main() {
    println!("Hello, world!");
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
    }

    #[test]
    fn test_parsing_rows() {
        let rows = "...123......$34.\n12..76........!.";

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
}
