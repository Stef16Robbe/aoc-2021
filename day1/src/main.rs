use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut larger_meas_count: u32 = 0;
    let mut previous: i32 = -1;
    
    for line in file.split("\n") {
        if line == "" { continue; }
        let measurement = line.parse::<i32>().unwrap();
        if measurement > previous && previous != -1 {
            larger_meas_count += 1;
        }
        previous = measurement;
    }

    println!("{}", larger_meas_count)
    // 1342

}
