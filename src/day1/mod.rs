use crate::read_input::read_input;
use anyhow::Result;

pub fn first_part() -> Result<()> {
    let lines = read_input("day1")?;
    let sum = lines.fold(0, |acc, line| {
        let line = line.expect("Failed to read line");
        let left = line
            .chars()
            .find_map(|char| char.to_digit(10))
            .expect("String must contains two digits");
        let right = line
            .chars()
            .rev()
            .find_map(|char| char.to_digit(10))
            .expect("String must contains two digits");

        acc + left * 10 + right
    });
    println!("Sum: {sum}");
    Ok(())
}

pub fn second_part() -> Result<()> {
    let lines = read_input("day1")?;
    let sum = lines.fold(0, |acc, line| {
        let line = line.expect("Failed to read line");
        let left = find_digit(line.as_str());
        let right = find_digit_rev(line.as_str());
        acc + left * 10 + right
    });
    println!("Sum: {sum}");
    Ok(())
}

fn find_digit(line: &str) -> u32 {
    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }
        if let Some(digit) = is_digit(&line[index..]) {
            return digit;
        }
    }
    panic!("Expect digit.")
}

fn is_digit(str: &str) -> Option<u32> {
    let mut chars = str.chars();
    match chars.next()? {
        'o' if contains(chars.clone(), "ne") => Some(1),
        't' if contains(chars.clone(), "wo") => Some(2),
        't' if contains(chars.clone(), "hree") => Some(3),
        'f' if contains(chars.clone(), "our") => Some(4),
        'f' if contains(chars.clone(), "ive") => Some(5),
        's' if contains(chars.clone(), "ix") => Some(6),
        's' if contains(chars.clone(), "even") => Some(7),
        'e' if contains(chars.clone(), "ight") => Some(8),
        'n' if contains(chars.clone(), "ine") => Some(9),
        _ => None,
    }
}

fn find_digit_rev(line: &str) -> u32 {
    let end_index = line.len();
    for (index, char) in line.chars().rev().enumerate() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }
        if let Some(digit) = is_digit_rev(&line[..end_index - index]) {
            return digit;
        }
    }
    panic!("Expect digit.")
}

fn is_digit_rev(str: &str) -> Option<u32> {
    let mut chars = str.chars().rev();
    match chars.next()? {
        'e' if contains_rev(chars.clone(), "on") => Some(1),
        'o' if contains_rev(chars.clone(), "tw") => Some(2),
        'e' if contains_rev(chars.clone(), "thre") => Some(3),
        'r' if contains_rev(chars.clone(), "fou") => Some(4),
        'e' if contains_rev(chars.clone(), "fiv") => Some(5),
        'x' if contains_rev(chars.clone(), "si") => Some(6),
        'n' if contains_rev(chars.clone(), "seve") => Some(7),
        't' if contains_rev(chars.clone(), "eigh") => Some(8),
        'e' if contains_rev(chars.clone(), "nin") => Some(9),
        _ => None,
    }
}

fn contains(chars: impl Iterator<Item = char>, str: &str) -> bool {
    str.chars().zip(chars).all(|(left, right)| left == right)
}

fn contains_rev(chars: impl Iterator<Item = char>, str: &str) -> bool {
    str.chars()
        .rev()
        .zip(chars)
        .all(|(left, right)| left == right)
}
