use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_risk_level(readings: Vec<Vec<u8>>) -> u64 {
    let mut low_points = 0u64;

    for (i, row) in readings.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let current = readings[i][j];
            let mut compare_to: Vec<u8> = vec![];
            let mut smaller = true;

            if i > 0 {
                compare_to.push(readings[i - 1][j]);
            }
            if j > 0 {
                compare_to.push(readings[i][j - 1]);
            }
            if i + 1 < readings.len() {
                compare_to.push(readings[i + 1][j]);
            }
            if j + 1 < row.len() {
                compare_to.push(readings[i][j + 1]);
            }

            for c in compare_to {
                if c <= current { smaller = false; }
            }

            if smaller { low_points += (readings[i][j] + 1) as u64 }
        }
    }

    low_points
}

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    
    let mut readings: Vec<Vec<u8>> = vec![];

    for line in file.lines() {
        // https://stackoverflow.com/a/66590374/10503012
        let nums: Vec<u8> = line
            .unwrap()
            .chars()
            .map(|n| n as u8 - 0x30)
            .collect();

        readings.push(nums);
    }

    let low_points = get_risk_level(readings);
    println!("{:?}", low_points);

}

/*
TODO:
- read file into 2d array
- for each num in array, compare it with (x - 1, y), (x + 1, y) (x, y + 1), (x, y - 1)
- if lower than all of those, add to list
- loop final list, +1 to each, sum is result
*/

