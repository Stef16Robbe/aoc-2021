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
    amount: i32
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let movement: Vec<&str> = s.split(' ').collect();

        let direction_fromstr = movement[0].parse::<Direction>().unwrap();
        let amount_fromstr = movement[1].parse::<i32>()?;

        Ok(Movement { direction: direction_fromstr, amount: amount_fromstr })
    }
}

// https://cotigao.medium.com/mutable-reference-in-rust-995320366e22
fn move_forward(horizontal: &mut i32, depth: &mut i32, aim: &i32, amount: &i32) {
    *horizontal += amount;
    *depth += aim * amount;
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut horizontal = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for line in file.split("\n") {
        if line == "" { continue; }
        let movement = line.parse::<Movement>().unwrap();

        match movement.direction {
            Direction::Forward => move_forward(&mut horizontal, &mut depth, &aim, &movement.amount),
            Direction::Down => aim += movement.amount as i32,
            Direction::Up => aim -= movement.amount as i32
        };

    }

    println!("{} * {} = {}", horizontal, depth, horizontal * depth)
    // 1857958050
}
