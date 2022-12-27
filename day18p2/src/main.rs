use std::collections::hash_set::{HashSet, Iter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Block {
    x: isize,
    y: isize,
    z: isize,
}

impl Block {
    fn neighbours(&self) -> HashSet<Block> {
        let mut neighbours = HashSet::new();

        neighbours.insert((self.x - 1, self.y, self.z).into());
        neighbours.insert((self.x + 1, self.y, self.z).into());

        neighbours.insert((self.x, self.y - 1, self.z).into());
        neighbours.insert((self.x, self.y + 1, self.z).into());

        neighbours.insert((self.x, self.y, self.z - 1).into());
        neighbours.insert((self.x, self.y, self.z + 1).into());

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

#[derive(Debug, Default)]
struct Blob {
    blocks: HashSet<Block>,
    max: Block,
}

impl Blob {
    fn insert(&mut self, block: Block) {
        if block.x > self.max.x {
            self.max.x = block.x;
        }

        if block.y > self.max.y {
            self.max.y = block.y;
        }

        if block.z > self.max.z {
            self.max.z = block.z;
        }

        self.blocks.insert(block);
    }

    fn contains(&self, block: impl Into<Block>) -> bool {
        self.blocks.contains(&block.into())
    }

    fn fill_voids(&mut self) {
        let mut outside: HashSet<Block> = HashSet::new();
        let mut void: HashSet<Block> = HashSet::new();

        for x in 0..=self.max.x {
            for y in 0..=self.max.y {
                for z in 0..=self.max.z {
                    let block: Block = (x, y, z).into();
                    if !self.blocks.contains(&block) {
                        void.insert(block);
                    }
                }
            }
        }

        let mut block_found = true;

        while block_found {
            block_found = false;

            for &block in &void {
                if block.x == 0 || block.y == 0 || block.z == 0 {
                    outside.insert(block);
                    block_found = true;
                } else if block.x == self.max.x || block.y == self.max.y || block.z == self.max.z {
                    outside.insert(block);
                    block_found = true;
                } else if !block.neighbours().is_disjoint(&outside) {
                    outside.insert(block);
                    block_found = true;
                }
            }

            void.retain(|b| !outside.contains(b));
        }

        self.blocks.extend(void);
    }
}

impl Extend<Block> for Blob {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Block>,
    {
        for block in iter {
            self.insert(block);
        }
    }
}

impl<'a> IntoIterator for &'a Blob {
    type Item = &'a Block;
    type IntoIter = Iter<'a, Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut blob = Blob::default();

    for line in input.lines() {
        let line = line.unwrap();

        let (x, y, z) = line
            .split(',')
            .map(|v| isize::from_str_radix(v, 10).unwrap())
            .collect_tuple()
            .unwrap();

        blob.insert((x, y, z).into());
    }

    // for z in 0..=blob.max.z {
    //     println!("z={z}");
    //     for y in 0..=blob.max.y {
    //         println!(
    //             "{}",
    //             (0..=blob.max.x)
    //                 .into_iter()
    //                 .map(|x| if blob.contains((x, y, z)) { "#" } else { "." })
    //                 .join("")
    //         );
    //     }
    //     println!("")
    // }

    blob.fill_voids();

    // for z in 0..=blob.max.z {
    //     println!("z={z}");
    //     for y in 0..=blob.max.y {
    //         println!(
    //             "{}",
    //             (0..=blob.max.x)
    //                 .into_iter()
    //                 .map(|x| if blob.contains((x, y, z)) { "#" } else { "." })
    //                 .join("")
    //         );
    //     }
    //     println!("")
    // }

    let mut exposed = 0;

    for block in &blob {
        exposed += block
            .neighbours()
            .into_iter()
            .map(|n| if blob.contains(n) { 0 } else { 1 })
            .sum::<isize>();
    }

    println!("Exposed faces: {exposed}");
}
