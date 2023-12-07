use crate::{day_solution::DaySolution, read_input::read_input};
use anyhow::Result;
use std::fmt::Display;

pub struct SeventhDaySolution;

impl DaySolution for SeventhDaySolution {
    fn first_part() -> Result<String> {
        let input = read_input("day7")?;
        let mut bids = input
            .map(|l| Bid::from(l.unwrap().as_str()))
            .collect::<Vec<_>>();
        bids.sort_by(|l, r| l.hand.cmp(&r.hand));
        let result = bids
            .into_iter()
            .enumerate()
            .map(|(index, bid)| (index as u32 + 1) * bid.bid)
            .sum::<u32>();
        Ok(format!("Result: {result}"))
    }

    fn second_part() -> Result<String> {
        Self::first_part()
    }
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl From<&str> for Bid {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        Self {
            hand: hand.into(),
            bid: bid.parse().unwrap(),
        }
    }
}

#[derive(Eq, Debug, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Num(u32),
}

impl Card {
    fn weight(&self) -> u32 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 1,
            Card::T => 10,
            Card::Num(num) => *num,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.weight().eq(&other.weight())
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        if value.is_ascii_digit() {
            return Self::Num(value.to_digit(10).unwrap());
        }
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => panic!("Unknown card {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cards([Card; 5]);

impl From<&str> for Cards {
    fn from(value: &str) -> Self {
        let mut cards = [Card::A; 5];
        value
            .chars()
            .map(|c| c.into())
            .enumerate()
            .for_each(|(index, card)| {
                cards[index] = card;
            });
        Self(cards)
    }
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = Cards::from(value);
        let hand_type = HandType::from(&cards);
        Self { cards, hand_type }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<&Cards> for HandType {
    fn from(value: &Cards) -> Self {
        let mut counts = [0; 15];
        for card in value.0 {
            let weight = card.weight();
            counts[weight as usize] += 1;
        }
        let joker_count = counts[1];
        counts[1] = 0;

        let mut counts = counts.into_iter().filter(|c| *c > 0).collect::<Vec<_>>();
        if counts.is_empty() {
            return Self::FiveOfKind;
        }
        counts.sort();
        let len = counts.len();
        counts[len - 1] += joker_count;

        match counts.len() {
            1 => Self::FiveOfKind,
            2 if counts[0] == 1 => Self::FourOfKind,
            2 => Self::FullHouse,
            3 if counts[2] == 3 => Self::ThreeOfKind,
            3 => Self::TwoPair,
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("Unexpected combination"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        self.cards.0 == other.cards.0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for (l, r) in self.cards.0.iter().zip(other.cards.0.iter()) {
            if l != r {
                return l.cmp(r);
            }
        }
        std::cmp::Ordering::Equal
    }
}
