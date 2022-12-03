use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut priority_sum = 0;

    for line in input.lines() {
        let line = line.unwrap();

        let compartment1: HashSet<char> = line[..line.len() / 2].chars().collect();
        let compartment2: HashSet<char> = line[line.len() / 2..].chars().collect();
        let common: Vec<char> = compartment1.intersection(&compartment2).cloned().collect();
        let common = common[0];

        let priority: u32 = common.try_into().unwrap();
        let priority = match priority {
            x if x >= 97 => x - 96,
            x if x >= 65 => x - 38,
            _ => panic!("eek"),
        };

        println!("Common item is '{common}' with priority {priority}");
        priority_sum += priority;
    }

    println!("Priority sum is {priority_sum}")
}
