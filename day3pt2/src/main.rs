use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("day3pt2/input.txt").expect("Error opening file");
    let reader = BufReader::new(file);
    
    let mut sum = 0;

    let mut buffer: Vec<Vec<char>> = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line: String = line_result.expect("Error reading line");

        let chars: Vec<char> = line.chars().collect();
        buffer.push(chars);

        if (i + 1) % 3 == 0 {
            let shared = intersection(&buffer);
            for (k, _) in shared {
                sum += priority(&k);
            }
            buffer.clear();
        }
    }       

    println!("Sum: {}", sum);
}

fn intersection(vecs: &Vec<Vec<char>>) -> HashMap<char, u8> {
    let mut result: HashMap<char, u8> = HashMap::new();

    for vec in vecs {
        let mut set: HashSet<char> = HashSet::new();
        for c in vec {
            set.insert(*c);
        }
        for c in set.iter() {
            *result.entry(*c).or_insert(0) += 1;
        }
    }

    result.retain(|_, v| *v == 3);
    result
}

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}
