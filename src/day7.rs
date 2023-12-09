use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;

#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<&Card> for u64 {
    fn from(card: &Card) -> Self {
        match card {
            Card::Joker => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        u64::from(self).cmp(&u64::from(other))
    }
}

impl From<char> for Card {
    fn from(s: char) -> Self {
        match s {
            'A' => Self::Ace,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            _ => panic!("Invalid card"),
        }
    }
}

impl From<&Hand> for u64 {
    fn from(hand: &Hand) -> Self {
        match hand {
            Hand::HighCard(_, _) => 1,
            Hand::OnePair(_, _) => 2,
            Hand::TwoPairs(_, _) => 3,
            Hand::ThreeOfAKind(_, _) => 4,
            Hand::FullHouse(_, _) => 5,
            Hand::FourOfAKind(_, _) => 6,
            Hand::FiveOfAKind(_, _) => 7,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        u64::from(self).cmp(&u64::from(other))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cmp(other) {
            Ordering::Equal => {
                let self_cards = match self {
                    Hand::HighCard(cards, _) => cards,
                    Hand::OnePair(cards, _) => cards,
                    Hand::TwoPairs(cards, _) => cards,
                    Hand::ThreeOfAKind(cards, _) => cards,
                    Hand::FullHouse(cards, _) => cards,
                    Hand::FourOfAKind(cards, _) => cards,
                    Hand::FiveOfAKind(cards, _) => cards,
                };

                let other_cards = match other {
                    Hand::HighCard(cards, _) => cards,
                    Hand::OnePair(cards, _) => cards,
                    Hand::TwoPairs(cards, _) => cards,
                    Hand::ThreeOfAKind(cards, _) => cards,
                    Hand::FullHouse(cards, _) => cards,
                    Hand::FourOfAKind(cards, _) => cards,
                    Hand::FiveOfAKind(cards, _) => cards,
                };

                for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
                    match self_card.cmp(other_card) {
                        Ordering::Equal => continue,
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Less => return Some(Ordering::Less),
                    }
                }

                None
            }
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    HighCard([Card; 5], u64),
    OnePair([Card; 5], u64),
    TwoPairs([Card; 5], u64),
    ThreeOfAKind([Card; 5], u64),
    FullHouse([Card; 5], u64),
    FourOfAKind([Card; 5], u64),
    FiveOfAKind([Card; 5], u64),
}

trait IntoHand {
    fn into_hand(self, bet: u64) -> Hand;
}

impl IntoHand for [Card; 5] {
    fn into_hand(self, bet: u64) -> Hand {
        let cards = self.to_vec();

        assert_eq!(cards.len(), 5);

        let count_jokers = cards.iter().filter(|card| **card == Card::Joker).count();

        let mut counts = HashMap::new();
        for card in cards.iter() {
            if card == &Card::Joker {
                continue;
            }
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }

        let count_pairs = counts.values().filter(|&&v| v == 2).count();
        let count_triples = counts.values().filter(|&&v| v == 3).count();
        let count_quads = counts.values().filter(|&&v| v == 4).count();
        let count_quints = counts.values().filter(|&&v| v == 5).count();

        let cards = [
            cards[0].clone(),
            cards[1].clone(),
            cards[2].clone(),
            cards[3].clone(),
            cards[4].clone(),
        ];

        match count_jokers {
            5 => return Hand::FiveOfAKind(cards, bet),
            4 => return Hand::FiveOfAKind(cards, bet),
            3 => {
                if count_pairs == 1 {
                    return Hand::FiveOfAKind(cards, bet);
                } else {
                    return Hand::FourOfAKind(cards, bet);
                }
            }
            2 => {
                if count_triples == 1 {
                    return Hand::FiveOfAKind(cards, bet);
                } else if count_pairs == 1 {
                    return Hand::FourOfAKind(cards, bet);
                } else {
                    return Hand::ThreeOfAKind(cards, bet);
                }
            }
            1 => {
                if count_quads == 1 {
                    return Hand::FiveOfAKind(cards, bet);
                } else if count_triples == 1 {
                    return Hand::FourOfAKind(cards, bet);
                } else if count_pairs == 2 {
                    return Hand::FullHouse(cards, bet);
                } else if count_pairs == 1 {
                    return Hand::ThreeOfAKind(cards, bet);
                } else {
                    return Hand::OnePair(cards, bet);
                }
            }
            _ => {}
        }

        match (count_pairs, count_triples, count_quads, count_quints) {
            (0, 0, 0, 0) => Hand::HighCard(cards, bet),
            (1, 0, 0, 0) => Hand::OnePair(cards, bet),
            (2, 0, 0, 0) => Hand::TwoPairs(cards, bet),
            (0, 1, 0, 0) => Hand::ThreeOfAKind(cards, bet),
            (1, 1, 0, 0) => Hand::FullHouse(cards, bet),
            (0, 0, 1, 0) => Hand::FourOfAKind(cards, bet),
            (0, 0, 0, 1) => Hand::FiveOfAKind(cards, bet),
            _ => {
                panic!("Invalid hand")
            }
        }
    }
}

fn solve(mut hands: Vec<Hand>) -> u64 {
    hands.sort();

    let file = std::fs::File::create("src/assets/day7/output").unwrap();
    let mut file = std::io::BufWriter::new(file);
    for hand in &hands {
        file.write_all(format!("{:?}\n", hand).as_bytes()).unwrap();
    }

    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let index = index + 1;
        let bet = match hand {
            Hand::HighCard(_, bet) => bet,
            Hand::OnePair(_, bet) => bet,
            Hand::TwoPairs(_, bet) => bet,
            Hand::ThreeOfAKind(_, bet) => bet,
            Hand::FullHouse(_, bet) => bet,
            Hand::FourOfAKind(_, bet) => bet,
            Hand::FiveOfAKind(_, bet) => bet,
        };

        (index as u64 * bet) + acc
    })
}

fn main() {
    let input = include_str!("assets/day7/input");

    let hands = input
        .lines()
        .map(|line| {
            let cards = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .get(0)
                .unwrap()
                .chars()
                .map(|c| Card::from(c))
                .collect::<Vec<_>>();

            let bet = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let cards = [
                cards[0].clone(),
                cards[1].clone(),
                cards[2].clone(),
                cards[3].clone(),
                cards[4].clone(),
            ];

            cards.into_hand(bet)
        })
        .collect::<Vec<_>>();

    let result = solve(hands);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "No longer works after part 1"]
    fn test_part1() {
        let input = include_str!("assets/day7/input_test");

        let hands = input
            .lines()
            .map(|line| {
                let mut cards = line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .chars()
                    .map(|c| Card::from(c))
                    .collect::<Vec<_>>();

                let bet = line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();

                let cards = [
                    cards[0].clone(),
                    cards[1].clone(),
                    cards[2].clone(),
                    cards[3].clone(),
                    cards[4].clone(),
                ];

                cards.into_hand(bet)
            })
            .collect::<Vec<_>>();

        let result = solve(hands);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("assets/day7/input_test");

        let hands = input
            .lines()
            .map(|line| {
                let mut cards = line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .chars()
                    .map(|c| Card::from(c))
                    .collect::<Vec<_>>();

                let bet = line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .get(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();

                let cards = [
                    cards[0].clone(),
                    cards[1].clone(),
                    cards[2].clone(),
                    cards[3].clone(),
                    cards[4].clone(),
                ];

                cards.into_hand(bet)
            })
            .collect::<Vec<_>>();

        let result = solve(hands);

        assert_eq!(result, 5905);
    }

    #[test]
    fn it_correctly_uses_5_jokers() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Joker,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FiveOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_4_jokers() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Two,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FiveOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_3_jokers_with_one_pair() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Two,
            Card::Two,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FiveOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_3_jokers_with_no_pair() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Joker,
            Card::Two,
            Card::Three,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FourOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_2_jokers_with_one_pair() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Three,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FourOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_2_jokers_with_one_triple() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Two,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FiveOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_2_jokers_with_no_pair() {
        let cards = [
            Card::Joker,
            Card::Joker,
            Card::Two,
            Card::Three,
            Card::Four,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::ThreeOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_1_joker_with_one_pair() {
        let cards = [
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Three,
            Card::Four,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::ThreeOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_1_joker_with_two_pairs() {
        let cards = [
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Three,
            Card::Three,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FullHouse(cards, 1));
    }

    #[test]
    fn it_correctly_uses_1_joker_with_one_triple() {
        let cards = [
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Two,
            Card::Three,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FourOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_1_joker_with_one_quad() {
        let cards = [
            Card::Joker,
            Card::Two,
            Card::Two,
            Card::Two,
            Card::Two,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::FiveOfAKind(cards, 1));
    }

    #[test]
    fn it_correctly_uses_1_joker_with_no_pair() {
        let cards = [
            Card::Joker,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
        ];

        let hand = cards.clone().into_hand(1);

        assert_eq!(hand, Hand::OnePair(cards, 1));
    }
}
