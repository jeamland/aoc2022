use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use cgmath::{Point2, Vector2};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut head = Point2::new(0, 0);
    let mut tail = Point2::new(0, 0);
    let mut visited = HashSet::from([tail.clone()]);

    println!(
        "Head: ({}, {}) Tail: ({}, {})",
        head.x, head.y, tail.x, tail.y
    );

    for line in input.lines() {
        let line = line.unwrap();

        let (direction, distance) = line.split(' ').collect_tuple().unwrap();
        let direction = match direction {
            "U" => Vector2::new(0, 1),
            "D" => Vector2::new(0, -1),
            "L" => Vector2::new(-1, 0),
            "R" => Vector2::new(1, 0),
            _ => panic!("unknown direction"),
        };
        let distance = usize::from_str_radix(distance, 10).unwrap();

        println!();
        println!("==> {line}");

        for _ in 0..distance {
            head += direction;

            tail += match (head.x - tail.x, head.y - tail.y) {
                (dx, dy) if dx == 0 && dy > 1 => Vector2::new(0, 1),
                (dx, dy) if dx > 0 && dy > 1 => Vector2::new(1, 1),
                (dx, dy) if dx > 1 && dy > 0 => Vector2::new(1, 1),
                (dx, dy) if dx > 1 && dy == 0 => Vector2::new(1, 0),
                (dx, dy) if dx > 0 && dy < -1 => Vector2::new(1, -1),
                (dx, dy) if dx > 1 && dy < 0 => Vector2::new(1, -1),
                (dx, dy) if dx == 0 && dy < -1 => Vector2::new(0, -1),
                (dx, dy) if dx < 0 && dy < -1 => Vector2::new(-1, -1),
                (dx, dy) if dx < -1 && dy < 0 => Vector2::new(-1, -1),
                (dx, dy) if dx < -1 && dy == 0 => Vector2::new(-1, 0),
                (dx, dy) if dx < 0 && dy > 1 => Vector2::new(-1, 1),
                (dx, dy) if dx < -1 && dy > 0 => Vector2::new(-1, 1),
                _ => Vector2::new(0, 0),
            };

            visited.insert(tail.clone());

            println!(
                "Head: ({}, {}) Tail: ({}, {})",
                head.x, head.y, tail.x, tail.y
            );
        }
    }

    println!();
    println!("Tail visited {} points", visited.len());
}
