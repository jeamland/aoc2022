use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Self::from((x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl<T: Into<Point>> Add<T> for Point {
    type Output = Point;

    fn add(self, rhs: T) -> Point {
        let rhs: Point = rhs.into();

        Point::from((self.x + rhs.x, self.y + rhs.y))
    }
}

impl PartialEq<(isize, isize)> for Point {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

#[derive(Clone, Copy, Debug)]
enum Object {
    Air,
    Rock,
    Sand,
    Source,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
            Self::Source => write!(f, "+"),
        }
    }
}

struct Cave {
    objects: HashMap<Point, Object>,
    min: Point,
    max: Point,
}

impl Cave {
    fn insert(&mut self, point: Point, object: Object) {
        if point.x - 1 < self.min.x {
            self.min.x = point.x - 1;
        }
        if point.x > self.max.x + 1 {
            self.max.x = point.x + 1;
        }
        if point.y - 1 < self.min.y {
            self.min.y = point.y - 1;
        }
        if point.y + 2 > self.max.y {
            self.max.y = point.y + 2;
        }

        self.objects.insert(point, object);
    }

    fn get(&self, point: impl Into<Point>) -> Object {
        let point: Point = point.into();

        if point.y >= self.max.y {
            Object::Rock
        } else {
            self.objects
                .get(&point.into())
                .copied()
                .unwrap_or(Object::Air)
        }
    }

    fn introduce_sand(&mut self) -> bool {
        let mut sand = Point::from((500, 0));

        loop {
            let mut moved = false;

            for vector in [(0, 1), (-1, 1), (1, 1)] {
                let target = sand + vector;

                if matches!(self.get(target), Object::Air) {
                    sand = target;
                    moved = true;
                    break;
                }
            }

            if !moved {
                break;
            }
        }

        if sand == (500, 0) {
            false
        } else {
            if sand.x - 1 < self.min.x {
                self.min.x = sand.x - 1;
            }
            if sand.x > self.max.x + 1 {
                self.max.x = sand.x + 1;
            }
            self.objects.insert(sand, Object::Sand);
            true
        }
    }
}

impl Default for Cave {
    fn default() -> Self {
        Cave {
            objects: HashMap::from([((500, 0).into(), Object::Source)]),
            min: (499, 0).into(),
            max: (502, 2).into(),
        }
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                write!(f, "{}", self.get((x, y)))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut cave = Cave::default();

    for line in input.lines() {
        let line = line.unwrap();
        let points: Vec<Point> = line.split(" -> ").map(|s| s.into()).collect();

        for (start, end) in points.into_iter().tuple_windows() {
            if start.x == end.x {
                for y in isize::min(start.y, end.y)..=isize::max(start.y, end.y) {
                    cave.insert(Point::from((start.x, y)), Object::Rock);
                }
            } else {
                for x in isize::min(start.x, end.x)..=isize::max(start.x, end.x) {
                    cave.insert(Point::from((x, start.y)), Object::Rock);
                }
            }
        }
    }

    println!("{cave}");
    let mut units = 0;

    while cave.introduce_sand() {
        units += 1;
    }

    println!("{cave}");
    println!("{} units of sand", units + 1);
}
