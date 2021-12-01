use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut larger_meas_count: u32 = 0;
    let mut measurements: Vec<u32> = Vec::with_capacity(3);
    
    for line in file.split("\n") {
        if line == "" { continue; }
        let measurement = line.parse::<u32>().unwrap();
        
        if measurements.len() < 3 {
            measurements.push(measurement);
            continue;
        };

        let mut measurements_sum: u32 = measurements.iter().sum();
        let previous = measurements_sum;

        measurements.remove(0);
        measurements.push(measurement);

        measurements_sum = measurements.iter().sum();
        let current = measurements_sum;

        if current > previous {
            println!("{:?} increased", measurement);
            larger_meas_count += 1;
        } else if current < previous {
            println!("{:?} decreased", measurement);
        } else {
            println!("{:?} no change", measurement);
        }
    }

    println!("{}", larger_meas_count)
    // 1378
}
