use anyhow::Result;
use day1::FirstDaySolution;
use day2::SecondDaySolution;
use day3::ThirdDaySolution;
use day4::FourthDaySolution;
use day5::FifthDaySolution;
use day6::SixDaySolution;
use day7::SeventhDaySolution;
use day_solution::DaySolution;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day_solution;
mod read_input;

fn main() -> Result<()> {
    let challenge_day = std::env::args()
        .nth(1)
        .expect("Usage: <challenge day> <challenge part>");

    let challenge_part = std::env::args()
        .nth(2)
        .expect("Usage: <challenge day> <challenge part>");

    match challenge_day.as_str() {
        "1" => handle_solution::<FirstDaySolution>(challenge_part.as_str()),
        "2" => handle_solution::<SecondDaySolution>(challenge_part.as_str()),
        "3" => handle_solution::<ThirdDaySolution>(challenge_part.as_str()),
        "4" => handle_solution::<FourthDaySolution>(challenge_part.as_str()),
        "5" => handle_solution::<FifthDaySolution>(challenge_part.as_str()),
        "6" => handle_solution::<SixDaySolution>(challenge_part.as_str()),
        "7" => handle_solution::<SeventhDaySolution>(challenge_part.as_str()),
        _ => panic!("Solution not done yet."),
    }
}

fn handle_solution<S: DaySolution>(part: &str) -> Result<()> {
    let result = match part {
        "1" => S::first_part(),
        "2" => S::second_part(),
        _ => panic!("Each challenge has only two parts."),
    }?;
    println!("{result}");
    Ok(())
}
