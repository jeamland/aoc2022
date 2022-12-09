use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use cgmath::{Point2, Vector2};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut rope: Vec<Point2<isize>> = std::iter::repeat_with(|| Point2::new(0, 0))
        .take(10)
        .collect();
    let mut visited = HashSet::from([rope[9].clone()]);

    for knot in &rope {
        print!("({}, {}) ", knot.x, knot.y);
    }
    println!();

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
            rope[0] += direction;

            for (a, b) in (0..10).tuple_windows() {
                let v = follow_vector(&rope[a], &rope[b]);
                rope[b] += v;
            }

            visited.insert(rope[9].clone());

            for knot in &rope {
                print!("({}, {}) ", knot.x, knot.y);
            }
            println!();
        }
    }

    println!();
    println!("Tail visited {} points", visited.len());
}

fn follow_vector(a: &Point2<isize>, b: &Point2<isize>) -> Vector2<isize> {
    match (a.x - b.x, a.y - b.y) {
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
    }
}
