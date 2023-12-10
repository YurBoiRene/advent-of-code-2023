use clap::Parser;
use educe::Educe;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let input_filename = Cli::parse().input;
    let result = run_puzzle_file(&input_filename);

    println!("Final: {result}");
}

fn run_puzzle_file(filename: &PathBuf) -> usize {
    run_puzzle(&std::fs::read_to_string(filename).unwrap())
}

fn run_puzzle(input: &str) -> usize {
    let mut deck: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();
    deck.sort();
    deck.iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bet)
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Value(u8),
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(ParseStrErr {});
        }

        s.chars().next().unwrap().try_into()
    }
}

impl TryFrom<char> for Card {
    type Error = ParseStrErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Joker),
            'T' => Ok(Card::Value(10)),
            '2'..='9' => Ok(Card::Value(u8::from_str(&value.to_string()).unwrap())),
            _ => Err(ParseStrErr {}),
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut freq = self
            .cards
            .iter()
            .filter(|card| !matches!(card, Card::Joker))
            .fold(HashMap::<Card, usize>::new(), |mut freq, card| {
                *freq.entry(*card).or_default() += 1;
                freq
            })
            .into_iter()
            .collect::<Vec<(Card, usize)>>();
        freq.sort_by(|a, b| b.1.cmp(&a.1));

        let num_jokers = self
            .cards
            .iter()
            .filter(|card| matches!(card, Card::Joker))
            .count();

        if let Some(value) = freq.get_mut(0) {
            value.1 += num_jokers
        }

        match freq.get(0) {
            None => Type::FiveKind(Card::Joker),
            Some((card, 5)) => Type::FiveKind(*card),
            Some((card, 4)) => Type::FourKind(*card),
            Some((card, 3)) => {
                if let (card2, 2) = freq[1] {
                    Type::FullHouse(*card, card2)
                } else {
                    Type::ThreeKind(*card)
                }
            }
            Some((card, 2)) => {
                if let (card2, 2) = freq[1] {
                    Type::TwoPair(*card, card2)
                } else {
                    Type::Pair(*card)
                }
            }
            Some((_, 1)) => Type::HighCard(*self.cards.iter().max().unwrap()),
            _ => panic!("Couldn't get type of hand!"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.get_type().cmp(&other.get_type());
        if !type_cmp.is_eq() {
            // One hand is a better type
            type_cmp
        } else {
            // Fall back to value
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(me, them)| me.cmp(them))
                .find(|ord| !ord.is_eq())
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().take(5).map(|c| c.try_into().unwrap()).collect();

        let bet = s.split_whitespace().last().unwrap().parse().unwrap();

        Ok(Hand { cards, bet })
    }
}

#[derive(Debug, Copy, Clone, Eq, Educe)]
#[educe(PartialOrd, Ord)]
enum Type {
    HighCard(#[educe(Ord(ignore))] Card),
    Pair(#[educe(Ord(ignore))] Card),
    TwoPair(#[educe(Ord(ignore))] Card, #[educe(Ord(ignore))] Card),
    ThreeKind(#[educe(Ord(ignore))] Card),
    FullHouse(#[educe(Ord(ignore))] Card, #[educe(Ord(ignore))] Card),
    FourKind(#[educe(Ord(ignore))] Card),
    FiveKind(#[educe(Ord(ignore))] Card),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::HighCard(_), Self::HighCard(_))
                | (Self::Pair(_), Self::Pair(_))
                | (Self::TwoPair(_, _), Self::TwoPair(_, _))
                | (Self::ThreeKind(_), Self::ThreeKind(_))
                | (Self::FullHouse(_, _), Self::FullHouse(_, _))
                | (Self::FourKind(_), Self::FourKind(_))
                | (Self::FiveKind(_), Self::FiveKind(_))
        )
    }
}

#[derive(Debug)]
struct ParseStrErr {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_result_test() {
        let result = run_puzzle_file(&PathBuf::from("tests/input"));
        assert_eq!(result, 243101568);
    }

    #[test]
    fn card_order() {
        assert_eq!(Card::Queen, Card::Queen);
        assert_eq!(Card::Value(5), Card::Value(5));

        assert!(Card::Ace > Card::King);
        assert!(Card::Joker < Card::Value(5));
        assert!(Card::Value(5) > Card::Value(2));
        assert!(Card::Value(4) < Card::Value(5));
        assert!(Card::from_str("4").unwrap() < Card::from_str("5").unwrap());
    }

    #[test]
    fn type_order() {
        let king = Card::King;
        let queen = Card::Queen;
        let five = Card::Value(5);
        let nine = Card::Value(9);
        assert_eq!(Type::HighCard(five), Type::HighCard(five));
        assert_eq!(Type::HighCard(nine), Type::HighCard(five));
        assert_eq!(Type::TwoPair(nine, five), Type::TwoPair(king, queen));

        assert!(Type::Pair(king) > Type::HighCard(king));
        assert!(Type::Pair(five) > Type::HighCard(king));
        assert!(Type::TwoPair(five, nine) > Type::Pair(king));
        assert!(Type::TwoPair(king, five) > Type::Pair(nine));

        assert!(!(Type::TwoPair(king, five) > Type::TwoPair(queen, nine)));
    }
}
