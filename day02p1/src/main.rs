use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let scores = HashMap::from([
        ("A X", ROCK + DRAW),
        ("A Y", PAPER + WIN),
        ("A Z", SCISSORS + LOSE),
        ("B X", ROCK + LOSE),
        ("B Y", PAPER + DRAW),
        ("B Z", SCISSORS + WIN),
        ("C X", ROCK + WIN),
        ("C Y", PAPER + LOSE),
        ("C Z", SCISSORS + DRAW),
    ]);
    let mut total_score = 0;

    for line in input.lines() {
        let line = line.unwrap();
        total_score += scores.get(line.as_str()).unwrap();
    }

    println!("The final score is: {total_score}",);
}
