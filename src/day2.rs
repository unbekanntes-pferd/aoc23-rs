struct Game {
    id: u64,
    sets: Vec<GameSet>,
}

impl Game {
    fn min_red(&self) -> u64 {
        self.sets.iter().map(|set| set.red).max().unwrap().unwrap_or(1)
    }

    fn min_blue(&self) -> u64 {
        self.sets.iter().map(|set| set.blue).max().unwrap().unwrap_or(1)
    }

    fn min_green(&self) -> u64 {
        self.sets.iter().map(|set| set.green).max().unwrap().unwrap_or(1)
    }

    fn game_power(&self) -> u64 {
        self.min_red() * self.min_blue() * self.min_green()
    }
}

struct GameSet {
    red: Option<u64>,
    blue: Option<u64>,
    green: Option<u64>,
}

impl GameSet {

    fn new(red: Option<u64>, blue: Option<u64>, green: Option<u64>) -> GameSet {
        GameSet {
            red: red,
            blue: blue,
            green: green,
        }
    }

    fn is_valid(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        let valid_red = self.red.map(|red| red <= max_red).unwrap_or(true);
        let valid_green = self.green.map(|green| green <= max_green).unwrap_or(true);
        let valid_blue = self.blue.map(|blue| blue <= max_blue).unwrap_or(true);

        valid_red && valid_green && valid_blue
    }
}

impl From<&str> for GameSet {
    fn from(s: &str) -> GameSet {
        let colors: Vec<(String, u64)> = s.split(",").map(|s| {
            let parts: Vec<_> = s.split(" ").filter(|part| part.len() > 0).collect();
            let color = parts.last().expect("invalid format").to_string();
            let value: u64 = parts
                .first()
                .expect("invalid format")
                .parse::<u64>()
                .expect("Invalid format");
            (color, value)
        }).collect();
        let red = colors.iter().find(|(color, _)| color == "red").map(|(_, value)| *value);
        let blue = colors.iter().find(|(color, _)| color == "blue").map(|(_, value)| *value);
        let green = colors.iter().find(|(color, _)| color == "green").map(|(_, value)| *value);

        GameSet::new(red, blue, green)
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Game {
        let id = s
            .split(":")
            .next()
            .expect("invalid format")
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let raw_sets = s.split(":").nth(1).expect("invalid format");
        let sets: Vec<_> = raw_sets.split(";").map(|s| GameSet::from(s)).collect();

        Game { id, sets }
    }
}

fn solve(input: &str) -> u64 {
    let games: Vec<_> = input.split("\n").map(|s| Game::from(s)).collect();
    games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| set.is_valid(12, 13, 14))
        })
        .map(|game| game.id)
        .sum::<u64>()
}

fn solve_part2(input: &str) -> u64 {
    let games: Vec<_> = input.split("\n").map(|s| Game::from(s)).collect();

    games.iter().map(|game| game.game_power()).sum::<u64>()
}

fn main() {
    let input = include_str!("assets/day2/input");
    let result = solve(input);
    println!("Sum of valid ids: {}", result);

    let result = solve_part2(input);
    println!("Sum of game powers: {}", result);
}


#[cfg(test)]
mod tests {
    use crate::{solve, solve_part2};


    #[test]
    fn it_works() {
        let input = include_str!("assets/day2/input_test");
        let result = solve(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn it_works_part2() {
        let input = include_str!("assets/day2/input_test");
        let result = solve_part2(input);
        assert_eq!(result, 2286);
    }

}