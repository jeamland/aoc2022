use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut calorie_totals = Vec::new();
    let mut current_total = 0;

    for line in input.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            let calories: usize = line.parse().unwrap();
            current_total += calories;
        } else {
            calorie_totals.push(current_total);
            current_total = 0;
        }
    }

    calorie_totals.push(current_total);

    println!(
        "The most calories is: {}",
        calorie_totals.iter().max().unwrap()
    );
}
