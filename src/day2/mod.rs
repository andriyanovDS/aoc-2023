use crate::read_input::read_input;
use anyhow::Result;

const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

pub fn first_part() -> Result<()> {
    let input = read_input("day2")?;
    let mut sum = 0;
    for line in input {
        let line = line?;
        let game = Game::from(line.as_str());
        if game.is_valid(RED_CUBES, GREEN_CUBES, BLUE_CUBES) {
            sum += game.id;
        }
    }
    println!("Sum of ids: {sum}");
    Ok(())
}

pub fn second_part() -> Result<()> {
    let input = read_input("day2")?;
    let mut sum = 0;
    for line in input {
        let line = line?;
        let game = Game::from(line.as_str());
        let power = game.blue * game.green * game.red;
        sum += power;
    }
    println!("Sum of ids: {sum}");
    Ok(())
}

struct Game {
    red: usize,
    green: usize,
    blue: usize,
    id: usize,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (id, subsets) = value[5..].split_once(": ").expect("Incorrect format.");
        let subsets = subsets.split(';');
        let mut game = Self {
            red: 0,
            green: 0,
            blue: 0,
            id: id.parse().expect("ID must be an integer."),
        };
        for subset in subsets {
            let passes = subset.split(',');
            for pass in passes {
                let (count, color) = pass.trim().split_once(' ').expect("Incorrect format.");
                let count = count.parse::<usize>().expect("Incorrect count");
                match color {
                    "red" => game.red = game.red.max(count),
                    "green" => game.green = game.green.max(count),
                    "blue" => game.blue = game.blue.max(count),
                    _ => panic!("Unexpected color"),
                }
            }
        }
        game
    }
}

impl Game {
    fn is_valid(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}
