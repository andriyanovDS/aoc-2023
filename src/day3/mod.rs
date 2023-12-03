use crate::read_input::read_input;
use anyhow::Result;
use std::{cmp::Ordering, ops::Range};

#[derive(Debug)]
struct Number {
    number: u32,
    row: usize,
    colums: Range<usize>,
}

#[derive(Debug)]
struct Symbol {
    column: usize,
}

pub fn first_part() -> Result<()> {
    let input = read_input("day3")?;
    let mut numbers = vec![];
    let mut symbols = vec![];
    for (row, line) in input.enumerate() {
        let line = line?;
        let mut number = None;
        let mut row_symbols = vec![];
        for (column, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number = Some(number.map(|n| n * 10).unwrap_or(0) + digit);
                continue;
            }
            if let Some(n) = number {
                let start = column - n.ilog10() as usize;
                numbers.push(Number {
                    number: n,
                    row,
                    colums: start.saturating_sub(2)..column + 1,
                });
                number = None;
            }
            if char != '.' {
                row_symbols.push(Symbol { column })
            }
        }
        if let Some(n) = number {
            let column = line.len();
            let start = column - n.ilog10() as usize;
            numbers.push(Number {
                number: n,
                row,
                colums: start.saturating_sub(2)..column + 1,
            });
        }
        symbols.push(row_symbols);
    }
    let sum = sum_of_numbers_adjustent_to_symbol(&numbers, &symbols);
    println!("Sum: {sum}");
    Ok(())
}

fn sum_of_numbers_adjustent_to_symbol(numbers: &[Number], symbols: &[Vec<Symbol>]) -> u32 {
    numbers
        .iter()
        .filter_map(|number| {
            let mut lines = Vec::with_capacity(3);
            if number.row > 0 {
                lines.push(&symbols[number.row - 1]);
            }
            lines.push(&symbols[number.row]);
            if number.row + 1 < symbols.len() {
                lines.push(&symbols[number.row + 1]);
            }
            lines.iter().find_map(|line| {
                line.binary_search_by(|s| compare(number, s))
                    .ok()
                    .map(|_| number.number)
            })
        })
        .sum()
}

fn compare(number: &Number, symbol: &Symbol) -> Ordering {
    if number.colums.contains(&symbol.column) {
        Ordering::Equal
    } else if number.colums.start < symbol.column {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[derive(Debug)]
struct Gear {
    column: usize,
    row: usize,
}

pub fn second_part() -> Result<()> {
    let input = read_input("day3")?;
    let mut numbers = vec![];
    let mut gears = vec![];
    for (row, line) in input.enumerate() {
        let line = line?;
        let mut number = None;
        let mut row_numbers = vec![];
        for (column, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number = Some(number.map(|n| n * 10).unwrap_or(0) + digit);
                continue;
            }
            if let Some(n) = number {
                let start = column - n.ilog10() as usize;
                row_numbers.push(Number {
                    number: n,
                    row,
                    colums: start.saturating_sub(2)..column + 1,
                });
                number = None;
            }
            if char == '*' {
                gears.push(Gear { column, row })
            }
        }
        if let Some(n) = number {
            let column = line.len();
            let start = column - n.ilog10() as usize;
            row_numbers.push(Number {
                number: n,
                row,
                colums: start.saturating_sub(2)..column + 1,
            });
        }
        numbers.push(row_numbers);
    }
    let sum = gear_ratios(&numbers, &gears);
    println!("Ratios: {sum}");
    Ok(())
}

fn gear_ratios(numbers: &[Vec<Number>], gears: &[Gear]) -> usize {
    gears
        .iter()
        .filter_map(|gear| {
            let mut lines = Vec::with_capacity(3);
            if gear.row > 0 {
                lines.push(&numbers[gear.row - 1]);
            }
            lines.push(&numbers[gear.row]);
            if gear.row + 1 < numbers.len() {
                lines.push(&numbers[gear.row + 1]);
            }
            let numbers = lines
                .into_iter()
                .flat_map(|line| {
                    line.iter()
                        .filter(|n| n.colums.contains(&gear.column))
                        .map(|n| n.number)
                })
                .collect::<Vec<u32>>();
            if numbers.len() == 2 {
                Some(numbers[0] as usize * numbers[1] as usize)
            } else {
                None
            }
        })
        .sum()
}
