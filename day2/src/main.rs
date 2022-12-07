use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    /* 
     * A = rock
     * B = paper
     * C = scissors
     *
     * X = rock
     * Y = paper
     * z = scissors
     */

    let file = File::open("day2/input.txt").expect("Error opening file");
    let reader = BufReader::new(file);

    let mut score: u128 = 0;

    for line_result in reader.lines() {
        let line: String = line_result.expect("Error reading line");
        let split: Vec<&str> = line.split_whitespace().collect();

        score += game_score(line.as_str()) + shape_score(split[1]);
    }

    println!("Total score: {}", score);
}

fn game_score(play: &str) -> u128 {
    match play {
        "A X" => 3,
        "A Y" => 6,
        "A Z" => 0,
        "B X" => 0,
        "B Y" => 3,
        "B Z" => 6,
        "C X" => 6,
        "C Y" => 0,
        "C Z" => 3,
        &_ => 0,
    }   
}

fn shape_score(shape: &str) -> u128 {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0,
    }
}

