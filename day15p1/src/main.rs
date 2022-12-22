use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
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
        let (_, coords) = s.split_once("x=").unwrap();
        let (x, y) = coords.split_once(", y=").unwrap();
        Self::from((x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Default)]
struct RangeSet {
    ranges: Vec<Range<isize>>,
}

impl RangeSet {
    fn add(&mut self, start: isize, end: isize) {
        self.ranges.push(start..end + 1);
        self.ranges.sort_unstable_by_key(|r| r.start);

        let mut changed = true;

        while changed {
            changed = false;

            for idx in 0..self.ranges.len() - 1 {
                if self.ranges[idx].end < self.ranges[idx + 1].start {
                    continue;
                }

                let end = isize::max(self.ranges[idx].end, self.ranges[idx + 1].end);
                self.ranges[idx].end = end;
                self.ranges.remove(idx + 1);

                changed = true;
                break;
            }
        }
    }

    fn len(&self) -> usize {
        self.ranges.iter().map(|r| (r.end - r.start) as usize).sum()
    }
}

// const GOAL_ROW: isize = 10;
const GOAL_ROW: isize = 2000000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut unreachables = RangeSet::default();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let line = line.unwrap();

        let (sensor, beacon) = line.split_once(": ").unwrap();
        let sensor = Point::from(sensor);
        let beacon = Point::from(beacon);

        let distance = sensor.manhattan_distance(&beacon);
        let y_distance = sensor.y.abs_diff(GOAL_ROW);

        println!("S @ {sensor}, B @ {beacon} -> {distance} ({y_distance})");

        if y_distance <= distance {
            let x_distance = (distance - y_distance) as isize;
            unreachables.add(sensor.x - x_distance, sensor.x + x_distance);
        }

        if beacon.y == GOAL_ROW {
            beacons.insert(beacon.x);
        }
    }

    println!(
        "No beacons at {} points",
        unreachables.len() - beacons.len()
    );
}
