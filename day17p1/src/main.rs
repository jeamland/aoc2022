use std::collections::HashSet;
use std::fmt;
use std::iter::Cycle;
use std::str::Chars;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Minus,
    Plus,
    L,
    Pipe,
    Square,
}

impl Shape {
    fn next(&self) -> Self {
        match self {
            Self::Minus => Self::Plus,
            Self::Plus => Self::L,
            Self::L => Self::Pipe,
            Self::Pipe => Self::Square,
            Self::Square => Self::Minus,
        }
    }

    fn occupied_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            Self::Minus => {
                vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
            }
            Self::Plus => {
                vec![
                    (x + 1, y + 2),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 2, y + 1),
                    (x + 1, y),
                ]
            }
            Self::L => vec![
                (x + 2, y + 2),
                (x + 2, y + 1),
                (x, y),
                (x + 1, y),
                (x + 2, y),
            ],
            Self::Pipe => vec![(x, y + 3), (x, y + 2), (x, y + 1), (x, y)],
            Self::Square => vec![(x, y + 1), (x + 1, y + 1), (x, y), (x + 1, y)],
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::Minus => 4,
            Self::Plus => 3,
            Self::L => 3,
            Self::Pipe => 1,
            Self::Square => 2,
        }
    }
}

#[derive(Default)]
struct Shaft {
    shaft: Vec<Vec<bool>>,
    occupied_points: HashSet<(usize, usize)>,
}

impl Shaft {
    fn drop(&mut self, shape: Shape, jets: &mut Cycle<Chars<'_>>) {
        let mut x = 2;
        let mut y = self.shaft.len() + 3;

        for _ in 0..=y {
            let jet = jets.next().unwrap();

            let mut new_x = x;
            if jet == '<' && x > 0 {
                new_x -= 1;
            } else if jet == '>' && x + shape.width() < 7 {
                new_x += 1;
            };

            if new_x != x
                && shape
                    .occupied_points(new_x, y)
                    .iter()
                    .all(|p| !self.occupied_points.contains(p))
            {
                x = new_x;
            }

            if y == 0 {
                break;
            }

            y -= 1;

            if shape
                .occupied_points(x, y)
                .iter()
                .any(|p| self.occupied_points.contains(p))
            {
                y += 1;
                break;
            }
        }

        for (x, y) in shape.occupied_points(x, y) {
            while y >= self.shaft.len() {
                self.shaft.push(vec![false; 7]);
            }

            self.shaft[y][x] = true;
            self.occupied_points.insert((x, y));
        }
    }

    fn height(&self) -> usize {
        self.shaft.len()
    }
}

impl fmt::Display for Shaft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, row) in self.shaft.iter().enumerate().rev() {
            writeln!(
                f,
                "{:4} |{}|",
                index,
                row.iter().map(|&b| if b { "#" } else { "." }).join("")
            )?;
        }

        write!(f, "     ---------")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let mut jets = input.trim_end().chars().cycle();

    let mut shaft = Shaft::default();
    let mut shape = Shape::Minus;

    for _ in 0..2022 {
        shaft.drop(shape, &mut jets);
        shape = shape.next();
    }

    println!("Height is {}", shaft.height());
}
