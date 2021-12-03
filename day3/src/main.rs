use std::fs;

fn edit_entries(entries: &mut Vec<Vec<char>>, more: bool) {
    for i in 0..entries[0].len() {
        if entries.len() == 1 {
            return;
        }

        let bits: Vec<char> = entries.iter().map(|val| val[i]).collect();

        // https://stackoverflow.com/a/55969946/10503012
        let zeroes: usize = bits.iter().filter(|bit| **bit == '0').count();
        let ones: usize = bits.iter().filter(|bit| **bit == '1').count();

        let higher_count = if zeroes > ones {
            0u32
        } else {
            1u32
        };
    
        if more {
            // https://stackoverflow.com/questions/41380367/parsing-a-char-to-u32/41380557#41380557
            entries.retain(|e| e[i].to_digit(10).unwrap() == higher_count);
        } else {
            entries.retain(|e| e[i].to_digit(10).unwrap() != higher_count);
        };
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut ratings: Vec<Vec<char>> = vec![];
    
    for line in file.split("\n") {
        if line == "" { continue; }
        ratings.push(line.chars().collect::<Vec<char>>());
    };

    let mut oxygen_ratings = ratings.to_vec();
    let mut scrubber_ratings = ratings.to_vec();
    edit_entries(&mut oxygen_ratings, true);
    edit_entries(&mut scrubber_ratings, false);

    // https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
    let oxygen_str = oxygen_ratings[0].iter().collect::<String>();
    let scrubber_str = scrubber_ratings[0].iter().collect::<String>();
    let oxygen = isize::from_str_radix(&oxygen_str, 2).unwrap() as u32;
    let scrubber = isize::from_str_radix(&scrubber_str, 2).unwrap() as u32;

    println!("oxygen: {} * scrubber: {} = {}", oxygen_str, scrubber_str, oxygen * scrubber)

    // 6124992
}
