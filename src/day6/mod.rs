use crate::{day_solution::DaySolution, read_input::read_input};

pub struct SixDaySolution;

impl DaySolution for SixDaySolution {
    fn first_part() -> anyhow::Result<String> {
        let mut input = read_input("day6")?;
        let time = input.next().unwrap()?;
        let distance = input.next().unwrap()?;
        let result = split(time.as_str())
            .into_iter()
            .zip(split(distance.as_str()).into_iter())
            .map(|(t, d)| handle_race(t, d))
            .fold(1, |acc, count| acc * count);

        Ok(format!("Result: {result}"))
    }

    fn second_part() -> anyhow::Result<String> {
        let mut input = read_input("day6")?;
        let time = input.next().unwrap()?;
        let distance = input.next().unwrap()?;
        let result = handle_race(parse(time.as_str()), parse(distance.as_str()));
        Ok(format!("Result: {result}"))
    }
}

fn handle_race(time: usize, distance: usize) -> usize {
    (1..time).fold(0, |acc, time_left| {
        let diff = time - time_left;
        if diff * time_left > distance {
            acc + 1
        } else {
            acc
        }
    })
}

fn split(line: &str) -> Vec<usize> {
    let (_, nums) = line.split_once(": ").unwrap();
    nums.split_whitespace()
        .filter_map(|n| {
            let num = n.trim();
            if num.is_empty() {
                None
            } else {
                Some(num.parse().unwrap())
            }
        })
        .collect()
}

fn parse(line: &str) -> usize {
    let (_, nums) = line.split_once(": ").unwrap();
    nums.split_whitespace().fold(0, |acc, n| {
        let num = n.trim();
        if num.is_empty() {
            acc
        } else {
            acc * 10usize.pow(n.len() as u32) + n.parse::<usize>().unwrap()
        }
    })
}
