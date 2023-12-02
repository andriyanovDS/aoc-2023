use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use anyhow::Result;

pub fn read_input(directory: &str) -> Result<Lines<BufReader<File>>> {
    let mut input_path = std::env::current_dir()?;
    input_path.push("src");
    input_path.push(directory);
    input_path.push("input.txt");

    let input = std::fs::File::open(&input_path)?;
    Ok(BufReader::new(input).lines())
}
