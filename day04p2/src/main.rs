use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut overlaps = 0;

    for line in input.lines() {
        let line = line.unwrap();

        let (first, second) = line
            .split(',')
            .map(|r| {
                let (start, end) = r
                    .split('-')
                    .map(|n| usize::from_str_radix(n, 10).unwrap())
                    .collect_tuple()
                    .unwrap();
                (start..=end).collect::<HashSet<usize>>()
            })
            .collect_tuple()
            .unwrap();

        if !first.is_disjoint(&second) {
            overlaps += 1;
        }
    }

    println!("Overlapping assignments: {overlaps}")
}
