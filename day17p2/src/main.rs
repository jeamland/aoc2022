use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::{Cycle, Peekable};
use std::str::Chars;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

type State = (Shape, char, Vec<Vec<bool>>);

#[derive(Default)]
struct Shaft {
    shaft: Vec<Vec<bool>>,
    occupied_points: HashSet<(usize, usize)>,

    rock_count: usize,
    states: HashMap<State, (usize, usize)>,
}

impl Shaft {
    fn drop(
        &mut self,
        shape: Shape,
        jets: &mut Peekable<Cycle<Chars<'_>>>,
    ) -> Option<(usize, usize, Vec<Vec<bool>>)> {
        self.rock_count += 1;

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

        let state: State = (
            shape,
            *jets.peek().unwrap(),
            self.shaft.iter().rev().take(30).rev().cloned().collect(),
        );

        if let Some(&(r0, height0)) = self.states.get(&state) {
            return Some((r0, height0, state.2));
        }

        self.states
            .insert(state.clone(), (self.rock_count, self.height()));
        None
    }

    fn height(&self) -> usize {
        self.shaft.len()
    }

    fn resync(&mut self, rows: Vec<Vec<bool>>) {
        let mut y = 0;

        for row in rows {
            for (x, &occupied) in row.iter().enumerate() {
                if occupied {
                    self.occupied_points.insert((x, y));
                }
            }
            self.shaft.push(row);

            y += 1;
        }
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
    const TOTAL_ROCKS: usize = 1000000000000;

    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let jets = input.trim_end();
    let mut jets = jets.chars().cycle().peekable();

    let mut shaft = Shaft::default();
    let mut shape = Shape::Minus;
    let mut cycle = 0;

    let (r0, height0, rows) = loop {
        cycle += 1;

        let state = shaft.drop(shape, &mut jets);
        shape = shape.next();

        if let Some((r0, height0, rows)) = state {
            break (r0, height0, rows);
        }
    };

    let cycle_length = cycle - r0;
    let cycle_height = shaft.height() - height0;

    let cycles = (TOTAL_ROCKS - cycle) / cycle_length;

    println!("Cycle starts at {r0} (height {height0}) and runs for {cycle_length} (height {cycle_height})");
    println!("Estimate {cycles} remain");

    let height = shaft.height() + (cycles * cycle_height);
    let rocks = cycle + (cycles * cycle_length);

    println!("At rock {rocks} height should be {height}");

    let mut shaft = Shaft::default();
    shaft.resync(rows);
    let height1 = shaft.height();

    for _ in rocks..TOTAL_ROCKS {
        shaft.drop(shape, &mut jets);
        shape = shape.next();
    }

    println!("Final height: {}", height + shaft.height() - height1);
    // println!(
    //     "Height is {height} after {} rocks ({cycles} cycles)",
    //     cycles * cycle_length * 7
    // );

    // let remainder = TOTAL_ROCKS - (cycles * cycle_length * 7);
    // println!("{remainder} rocks remaining");

    // for _ in 0..remainder {
    //     shaft.drop(shape, &mut jets);
    //     shape = shape.next();
    // }

    // println!("Final height is {}", height + shaft.height() - last_height);
}
