use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("day1/input.txt").expect("Error opening file");
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut local_max = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Error reading line");
        if line.is_empty() {
            if local_max > max {
                max = local_max;
            }
            local_max = 0;
            continue;
        }
        local_max += line.parse::<i32>().expect("Error parsing int")
    }

    println!("Most calories: {}", max);
}
