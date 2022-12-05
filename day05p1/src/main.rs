use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());
    let mut lines = input.lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in &mut lines {
        let line = line.unwrap();

        if line.starts_with(" 1") {
            break;
        }

        for (stack, label) in line
            .chars()
            .chunks(4)
            .into_iter()
            .map(|i| i.skip(1).next().unwrap())
            .enumerate()
        {
            if stacks.len() == stack {
                stacks.push(Vec::new());
            }

            if label != ' ' {
                stacks[stack].push(label);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    for line in &mut lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (count, from, to) = line
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| usize::from_str_radix(n, 10).unwrap())
            .collect_tuple()
            .unwrap();

        for _ in 0..count {
            let label = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(label);
        }
    }

    let message: String = stacks.iter_mut().map(|s| s.pop().unwrap()).collect();
    println!("Message is: {message}");
}
