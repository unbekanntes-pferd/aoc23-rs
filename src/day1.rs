fn main() {
    let input = include_str!("assets/day1/input");
    let result = solve(input);

    println!("Result is {}", result);
}

fn solve(input: &str) -> u64 {
    let input = parse_input(input);
    input.iter().map(|num| *num as u64).sum()
}

fn parse_input(text: &str) -> Vec<u8> {
    text.lines()
        .map(parse_number)
        .collect()
}

fn parse_number(line: &str) -> u8 {
    let mut numeric_words = line.numeric_words();
    numeric_words.sort_by_key(|(_, index)| *index);

    let mut numeric_chars: Vec<NumericChar> = line
        .chars()
        .enumerate()
        .filter(|(_index, char)| char.is_numeric())
        .map(|(index, char)| (char, index))
        .collect();
    numeric_chars.sort_by_key(|(_, index)| *index);

    let first = parse_first_number(&numeric_words, &numeric_chars);
    let second = parse_last_number(&numeric_words, &numeric_chars);

    format!("{first}{second}")
        .parse()
        .expect("chars not numeric")
}

fn parse_first_number(numeric_words: &[NumericWord], numeric_chars: &[NumericChar]) -> u8 {
    match (
        numeric_chars.first().is_some(),
        numeric_words.first().is_some(),
    ) {
        (true, false) => numeric_chars
            .first()
            .expect("no numbers present")
            .to_number(),
        (false, true) => numeric_words
            .first()
            .expect("no numbers present")
            .to_number(),
        (true, true) => {
            if numeric_words.first().unwrap().1 < numeric_chars.first().unwrap().1 {
                numeric_words
                    .first()
                    .expect("no numbers present")
                    .to_number()
            } else {
                numeric_chars
                    .first()
                    .expect("no numbers present")
                    .to_number()
            }
        }
        (false, false) => unreachable!("no numbers should never be present"),
    }
}

fn parse_last_number(numeric_words: &[NumericWord], numeric_chars: &[NumericChar]) -> u8 {
    match (
        numeric_chars.last().is_some(),
        numeric_words.last().is_some(),
    ) {
        (true, false) => numeric_chars
            .last()
            .expect("no numbers present")
            .to_number(),
        (false, true) => numeric_words
            .last()
            .expect("no numbers present")
            .to_number(),
        (true, true) => {
            if numeric_words.last().unwrap().1 > numeric_chars.last().unwrap().1 {
                numeric_words
                    .last()
                    .expect("no numbers present")
                    .to_number()
            } else {
                numeric_chars
                    .last()
                    .expect("no numbers present")
                    .to_number()
            }
        }
        (false, false) => unreachable!("no numbers should never be present"),
    }
}

type NumericWord = (String, usize);
type NumericChar = (char, usize);

trait IntoNumber {
    fn to_number(&self) -> u8;
}

impl IntoNumber for NumericChar {
    fn to_number(&self) -> u8 {
        if !self.0.is_numeric() {
            panic!("invalid char")
        } else {
            self.0.to_string().parse().expect("invalid numeric number")
        }
    }
}

impl IntoNumber for NumericWord {
    fn to_number(&self) -> u8 {
        match self.0.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => unreachable!("numerics words already parsed correctly"),
        }
    }
}

trait NumericWords {
    fn numeric_words(&self) -> Vec<NumericWord>;
}

impl NumericWords for &str {
    fn numeric_words(&self) -> Vec<NumericWord> {
        const NUMBER_WORDS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        NUMBER_WORDS
            .iter()
            .flat_map(|word| {
                self.match_indices(word)
                .map(|(index, _)| (word.to_string(), index))
                .collect::<Vec<NumericWord>>()
            })
            .collect()
    }
}
