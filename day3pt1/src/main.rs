use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("day3pt1/input.txt").expect("Error opening file");
    let reader = BufReader::new(file);
    
    let mut sum = 0;

    for line_result in reader.lines() {
        let line: String = line_result.expect("Error reading line");
        let chars: Vec<char> = line.chars().collect();
        let types = intersection(&chars);

        for t in types.iter() {
            sum += priority(t);
        }
    }       

    println!("Sum: {}", sum);
}

fn intersection(chars: &Vec<char>) -> HashSet<char> {
    let mid = chars.len() / 2;
    let (left, right) = chars.split_at(mid);

    let mut set: HashSet<char> = HashSet::new();
    let mut result: HashSet<char> = HashSet::new();

    for c in left {
        set.insert(*c);
    }
    for c in right {
        if set.contains(c) {
            result.insert(*c);
        }
    }

    result
}

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}
