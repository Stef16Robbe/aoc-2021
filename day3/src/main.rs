use std::fs;

fn get_bit(entry: &String, pos: &usize) -> u8 {
    entry.chars().nth(*pos).unwrap().to_string().parse::<u8>().unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut amount_zero = 0u32;
    let mut amount_one = 0u32;
    let mut gamma_rate_str: String = "".to_string();
    let mut epsilon_rate_str: String = "".to_string();
    let mut entries: Vec<String> = Vec::new();
    let mut byte_size = 0u8;

    for line in file.split("\n") {
        if line == "" { continue; }
        
        if byte_size == 0 {
            byte_size = line.len() as u8;
        }

        let entry = line.parse::<String>().unwrap();
        entries.push(entry);
    };

    for i in 0..byte_size as usize {
        for entry in &entries {
            match get_bit(&entry, &i) {
                0 => amount_zero += 1,
                1 => amount_one += 1,
                _ => panic!("Something went seriously wrong while reading input.txt...")
            };
        };

        if amount_zero > amount_one {
            gamma_rate_str += "0";
            epsilon_rate_str += "1";
        } else {
            gamma_rate_str += "1";
            epsilon_rate_str += "0";
        };
        
        amount_zero = 0;
        amount_one = 0;
    };

    let gamma_rate = isize::from_str_radix(&gamma_rate_str, 2).unwrap() as u32;
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_str, 2).unwrap() as u32;
    let power_consumption: u32 = gamma_rate * epsilon_rate;

    println!("gamma * epsilon ({} * {}) = {}", gamma_rate, epsilon_rate, power_consumption)
    // 1071734
}
