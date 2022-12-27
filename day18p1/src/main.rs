use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Block {
    x: isize,
    y: isize,
    z: isize,
}

impl Block {
    fn neighbours(&self) -> Vec<Block> {
        let mut neighbours = Vec::new();

        neighbours.push((self.x - 1, self.y, self.z).into());
        neighbours.push((self.x + 1, self.y, self.z).into());

        neighbours.push((self.x, self.y - 1, self.z).into());
        neighbours.push((self.x, self.y + 1, self.z).into());

        neighbours.push((self.x, self.y, self.z - 1).into());
        neighbours.push((self.x, self.y, self.z + 1).into());

        neighbours
    }
}

impl From<(isize, isize, isize)> for Block {
    fn from(tuple: (isize, isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut blocks: HashSet<Block> = HashSet::new();

    for line in input.lines() {
        let line = line.unwrap();

        let (x, y, z) = line
            .split(',')
            .map(|v| isize::from_str_radix(v, 10).unwrap())
            .collect_tuple()
            .unwrap();

        blocks.insert((x, y, z).into());
    }

    let mut exposed = 0;

    for block in &blocks {
        exposed += block
            .neighbours()
            .into_iter()
            .map(|n| if blocks.contains(&n) { 0 } else { 1 })
            .sum::<isize>();
    }

    println!("Exposed faces: {exposed}");
}
