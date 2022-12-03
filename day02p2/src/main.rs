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
        ("A X", SCISSORS + LOSE),
        ("A Y", ROCK + DRAW),
        ("A Z", PAPER + WIN),
        ("B X", ROCK + LOSE),
        ("B Y", PAPER + DRAW),
        ("B Z", SCISSORS + WIN),
        ("C X", PAPER + LOSE),
        ("C Y", SCISSORS + DRAW),
        ("C Z", ROCK + WIN),
    ]);
    let mut total_score = 0;

    for line in input.lines() {
        let line = line.unwrap();
        total_score += scores.get(line.as_str()).unwrap();
    }

    println!("The final score is: {total_score}",);
}
