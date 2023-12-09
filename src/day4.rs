fn main() {
    // PART 2

    let input = include_str!("assets/day4/input");
    // remove first 8 chars from line and split to lines
    let input = input
        .lines()
        .map(|line| line.chars().skip(7).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    let game = input.into_game();

    let mut winning_cards = Vec::new();
    for (card_index, _) in game.own.iter().enumerate() {
        process_card(&game, card_index, &mut winning_cards);
    }

    println!("Result is {}", winning_cards.len());
}

fn process_card(game: &Game, card_index: usize, winning_cards: &mut Vec<Card>) {
    if card_index >= game.own.len() {
        return;
    }

    let card = &game.own[card_index];
    let count_winning_numbers = game
        .winners
        .get(card_index)
        .unwrap()
        .iter()
        .filter(|num| card.contains(num))
        .count();

    // Add the current card
    winning_cards.push(card.clone());

    // Recursively process the next cards based on count of winning numbers
    for index in 1..=count_winning_numbers {
        process_card(game, card_index + index, winning_cards);
    }
}

#[allow(dead_code)]
fn solve_part1() {
    let input = include_str!("assets/day4/input");

    // remove first 8 chars from line and split to lines
    let input = input
        .lines()
        .map(|line| line.chars().skip(7).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    let game = input.into_game();

    let mut points = 0;

    for (card_index, card) in game.own.iter().enumerate() {
        let count_winning_numbers = game
            .winners
            .get(card_index)
            .unwrap()
            .iter()
            .filter(|num| card.contains(num))
            .count();

        println!("winning numbers: {}", count_winning_numbers);

        let mut card_points = 0;

        if count_winning_numbers > 0 {
            card_points += 1;
        }

        for _ in 1..count_winning_numbers {
            card_points *= 2;
        }

        points += card_points;

        println!("Card {} wins {} points", card_index, card_points);
    }

    println!("Result is {}", points);
}

type Card = Vec<u16>;

struct Game {
    winners: Vec<Card>,
    own: Vec<Card>,
}

trait IntoGame {
    fn into_game(self) -> Game;
}

impl IntoGame for &str {
    fn into_game(self) -> Game {
        self.lines().fold(
            Game {
                winners: Vec::new(),
                own: Vec::new(),
            },
            |mut game, line| {
                if line.is_empty() {
                    return game;
                }

                let cards: Vec<_> = line.split('|').collect();

                let winners = cards
                    .first()
                    .unwrap()
                    .split(' ')
                    .filter(|num| !num.is_empty() && num.chars().all(|c| c.is_numeric()))
                    .map(|card| card.parse::<u16>().unwrap())
                    .collect::<Vec<_>>();
                let own = cards
                    .get(1)
                    .unwrap()
                    .split(' ')
                    .filter(|num| !num.is_empty())
                    .map(|card| card.parse::<u16>().unwrap())
                    .collect::<Vec<_>>();

                game.winners.push(winners);
                game.own.push(own);

                game
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_card, IntoGame};

    #[test]
    fn it_parses_correctly_to_game() {
        let input = include_str!("assets/day4/input_test");
        // remove first 8 chars from line and split to lines
        let input = input
            .lines()
            .map(|line| line.chars().skip(7).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        let game = input.into_game();

        assert_eq!(game.winners.len(), 6);
        assert_eq!(game.own.len(), 6);

        assert_eq!(game.winners.first().unwrap().len(), 5);
        assert_eq!(game.own.first().unwrap().len(), 8);
    }

    #[test]
    fn it_works_correctly() {
        let input = include_str!("assets/day4/input_test");

        // remove first 8 chars from line and split to lines
        let input = input
            .lines()
            .map(|line| line.chars().skip(7).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        let game = input.into_game();

        let mut points = 0;

        for (card_index, card) in game.own.iter().enumerate() {
            let count_winning_numbers = game
                .winners
                .get(card_index)
                .unwrap()
                .iter()
                .filter(|num| card.contains(num))
                .count();

            let mut card_points = 0;

            if count_winning_numbers > 0 {
                card_points += 1;
            }

            for _ in 1..count_winning_numbers {
                card_points *= 2;
            }

            points += card_points;
        }

        assert_eq!(points, 13);
    }

    #[test]
    fn it_works_correctly_again() {
        let input = include_str!("assets/day4/input_test");

        // remove first 8 chars from line and split to lines
        let input = input
            .lines()
            .map(|line| line.chars().skip(7).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        let game = input.into_game();

        let mut winning_cards = Vec::new();

        for (card_index, _) in game.own.iter().enumerate() {
            process_card(&game, card_index, &mut winning_cards);
        }

        assert_eq!(winning_cards.len(), 30);
    }
}
