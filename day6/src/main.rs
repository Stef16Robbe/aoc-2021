use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

// https://github.com/emilyskidsister/aoc/blob/main/p2021_06/src/lib.rs#L3
pub fn count_fishes(mut state: u8, days: usize, memo: &mut HashMap<(u8, usize), usize>) -> usize {
    if let Some(result) = memo.get(&(state, days)) {
        return *result;
    }

    let orig_state = state;
    let mut count = 1;
    for d in 0..days {
        if state == 0 {
            count += count_fishes(8, days - d - 1, memo);
            state = 7;
        }
        state -= 1;
    }

    memo.insert((orig_state, days), count);
    count
}


fn main() {
    let mut file = BufReader::new(File::open("input.txt").unwrap());
    let mut numbers = String::new();
    file.read_line(&mut numbers).unwrap();

    let fishes: Vec<u8>;

    fishes = numbers
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut count = 0;
    let mut memo = HashMap::new();
    for fish in &fishes {
        let count_from_fish = count_fishes(*fish, 256, &mut memo);
        count += count_from_fish;
    }

    println!("{:?}", count);
}
