use crate::read_input::read_input;
use anyhow::Result;
use std::collections::HashSet;

struct Card {
    number: usize,
    match_count: usize,
}

impl Card {
    fn score(&self) -> u32 {
        if self.match_count == 0 {
            0
        } else {
            2u32.pow(self.match_count as u32 - 1)
        }
    }

    fn count_copies(&self, cards: &[Card], counts: &mut Vec<u32>) {
        if self.match_count == 0 {
            return;
        }
        for number in self.number + 1..=self.number + self.match_count {
            let index = number - 1;
            let card = &cards[index];
            counts[index] += 1;
            card.count_copies(cards, counts);
        }
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (header, card) = value.split_once(": ").unwrap();
        let (winners, real) = card.split_once('|').unwrap();
        let number = header.split_whitespace().last().unwrap().parse().unwrap();
        let winners = winners
            .split(' ')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .collect::<HashSet<u32>>();

        let match_count = real
            .split(' ')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .filter(|r| winners.contains(r))
            .count();

        Self {
            number,
            match_count,
        }
    }
}

pub fn first_part() -> Result<()> {
    let input = read_input("day4")?;
    let score = input
        .map(|l| Card::from(l.unwrap().as_str()))
        .fold(0, |acc, card| acc + card.score());
    println!("Score: {score}");
    Ok(())
}

pub fn second_part() -> Result<()> {
    let input = read_input("day4")?;
    let cards = input
        .map(|l| Card::from(l.unwrap().as_str()))
        .collect::<Vec<_>>();
    let mut counts = vec![1; cards.len()];
    cards
        .iter()
        .for_each(|c| c.count_copies(&cards, &mut counts));
    let sum: u32 = counts.into_iter().sum();
    println!("Sum: {sum}");
    Ok(())
}
