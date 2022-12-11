use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut x: isize = 1;
    let mut cycle: usize = 0;
    let mut sum = 0;

    for line in input.lines() {
        let line = line.unwrap();

        if line == "noop" {
            cycle += 1;

            if cycle % 40 == 20 {
                println!(
                    "At cycle {cycle}, X = {x}, signal strength = {}",
                    cycle as isize * x
                );
                sum += cycle as isize * x;
            }
        } else {
            let (_, value) = line.split_once(' ').unwrap();

            cycle += 1;
            if cycle % 40 == 20 {
                println!(
                    "At cycle {cycle}, X = {x}, signal strength = {}",
                    cycle as isize * x
                );
                sum += cycle as isize * x;
            }
            cycle += 1;
            if cycle % 40 == 20 {
                println!(
                    "At cycle {cycle}, X = {x}, signal strength = {}",
                    cycle as isize * x
                );
                sum += cycle as isize * x;
            }

            x += value.parse::<isize>().unwrap();
        }

        if cycle == 220 {
            break;
        }
    }

    println!("Sum of signal strengths = {sum}")
}
