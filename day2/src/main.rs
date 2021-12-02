use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;
use strum_macros::EnumString;

// https://docs.rs/strum_macros/0.23.1/strum_macros/derive.EnumString.html
#[derive(Debug, EnumString)]
enum Direction {
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "up")]
    Up
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: u32
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let movement: Vec<&str> = s.split(' ').collect();

        let direction_fromstr = movement[0].parse::<Direction>().unwrap();
        let amount_fromstr = movement[1].parse::<u32>()?;

        Ok(Movement { direction: direction_fromstr, amount: amount_fromstr })
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut horizontal = 0u32;
    let mut depth = 0u32;

    for line in file.split("\n") {
        if line == "" { continue; }
        let movement = line.parse::<Movement>().unwrap();

        match movement.direction {
            Direction::Forward => horizontal += movement.amount,
            Direction::Down => depth += movement.amount,
            Direction::Up => depth -= movement.amount
        };

    }

    println!("{} * {} = {}", horizontal, depth, horizontal * depth)
    // 1693300
}
