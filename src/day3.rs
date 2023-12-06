// THIS IS A PORT OF https://gitlab.dracoon.com/pioneer/advent-of-code/vm/-/blob/main/src/main/kotlin/day/three/GearRatios.kt
// originally solved by VM

fn main() {
    let input = include_str!("assets/day3/input_test");

    let matrix = input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut numbers: Vec<(usize, Vec<char>)> = Vec::new();

    matrix.iter().enumerate().for_each(|(row_idx, row)| {
        let mut num_str = String::new();

        for (col_idx, val) in row.iter().enumerate() {
            if val.is_numeric() {
                num_str.push(*val);
            }

            if !val.is_numeric() || col_idx == row.len() - 1 || !row[col_idx + 1].is_numeric() {
                if !num_str.is_empty() {
                    let num = num_str.parse::<usize>().unwrap();
                    let num_len = num_str.len();
                    num_str.clear();

                    let mut neighbors = Vec::new();

                    // Right neighbor
                    if col_idx < row.len() - 1 && row[col_idx + 1].is_numeric() {
                        neighbors.push(row[col_idx + num_len]);
                    } else {
                        neighbors.push('.');
                    }

                    // Left neighbor
                    if col_idx >= num_len {
                        neighbors.push(row[col_idx - num_len]);
                    } else {
                        neighbors.push('.');
                    }

                    // Top neighbors
                    if row_idx > 0 {
                        for offset in col_idx + 1 - num_len..=col_idx {
                            neighbors.push(matrix[row_idx - 1].get(offset).copied().unwrap_or('.'));
                        }
                    }

                    // Bottom neighbors
                    if row_idx < matrix.len() - 1 {
                        for offset in col_idx + 1 - num_len..=col_idx {
                            neighbors.push(matrix[row_idx + 1].get(offset).copied().unwrap_or('.'));
                        }
                    }

                    numbers.push((num, neighbors));
                }
            }
        }
    });

    let mut sum = 0;

    numbers.iter().for_each(|(num, neighbors)| {
        if neighbors
            .iter()
            .any(|c| *c != '.' && c.is_ascii_punctuation())
        {
            sum += num
        }
    });

    println!("Result is {}", sum);

}
