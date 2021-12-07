use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_steps(n: u64, mut total: u64) -> u64 {
    total += n;
    if n == 0 { return total }
    calculate_steps(n - 1, total)
}

fn calculate_fuel_costs(crabs: &Vec<i32>, position: &i32) -> u64 {
    let mut fuel_cost: u64 = 0;

    for crab in crabs {
        // https://doc.rust-lang.org/core/?search=abs
        fuel_cost += calculate_steps((crab - position).abs() as u64, 0);
    }

    fuel_cost
}

// One might say this is not the most 
// efficient method...
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
    // 95476244
}
