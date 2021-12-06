use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut file = BufReader::new(File::open("input.txt").unwrap());
    let mut numbers = String::new();
    file.read_line(&mut numbers).unwrap();

    let mut fishes: Vec<u8>;

    fishes = numbers
        .trim()
        .split(",")
        .map(|p| p.parse().unwrap())
        .collect();

    // https://stackoverflow.com/a/57637602/10503012
    // https://doc.rust-lang.org/std/ops/struct.Range.html
    for _ in 0..80 {
        for fish in 0..fishes.len() {
            if fishes[fish] == 0 {
                fishes.push(8);
                fishes[fish] = 7;
            }
            fishes[fish] -= 1;
        }
    }

    println!("{:?}", fishes.len());
}
