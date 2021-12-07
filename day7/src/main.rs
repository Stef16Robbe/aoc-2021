use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_fuel_costs(crabs: &Vec<i32>, position: &i32) -> u64 {
    let mut fuel_cost: u64 = 0;

    for crab in crabs {
        // https://doc.rust-lang.org/core/?search=abs
        fuel_cost += (crab - position).abs() as u64;
    }

    fuel_cost
}

/*
"Dumb" method:
- get min and max from nums
- for min..max: calculate fuel cost for each option
*/
fn main() {
    let mut file = BufReader::new(File::open("input.txt").unwrap());
    let mut numbers = String::new();
    file.read_line(&mut numbers).unwrap();

    let crabs: Vec<i32>;

    crabs = numbers
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap() + 1;

    let mut costs: Vec<u64> = Vec::new();

    for i in min..max {
        let cost = calculate_fuel_costs(&crabs, &i);
        costs.push(cost);
    }

    println!("{:?}", costs.iter().min().unwrap());
}
