use std::collections::HashMap;

mod astar;

use astar::AStar;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn advance_x(&mut self) {
        self.x += 1;
    }

    fn advance_y(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

struct Map {
    elevation: HashMap<Point, u32>,

    start: Point,
    end: Point,

    max: Point,
}

impl FromIterator<char> for Map {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = char>,
    {
        let mut elevation = HashMap::new();
        let mut point = Point::from((0, 0));
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        let mut max_x = 0;
        let mut max_y = 0;

        for ch in iter {
            match ch {
                '\n' => point.advance_y(),
                'S' => {
                    elevation.insert(point, 0);
                    start = Some(point);
                    point.advance_x();
                }
                'E' => {
                    elevation.insert(point, 25);
                    end = Some(point);
                    point.advance_x();
                }
                ch if ch >= 'a' && ch <= 'z' => {
                    elevation.insert(point, ch as u32 - 97);
                    point.advance_x();
                }
                _ => panic!("unexpected character"),
            }

            if point.x > max_x {
                max_x = point.x
            };
            if point.y > max_y {
                max_y = point.y
            };
        }

        let start = start.unwrap();
        let end = end.unwrap();

        Map {
            elevation,
            start,
            end,
            max: Point::from((max_x, max_y)),
        }
    }
}

impl AStar for Map {
    type Node = Point;
    type Weight = usize;

    fn heuristic(&self, from: Point, to: Point) -> usize {
        from.manhattan_distance(&to)
    }

    fn weight(&self, _from: Point, _to: Point) -> usize {
        1
    }

    fn neighbours(&self, node: Point) -> Vec<Point> {
        let mut points = Vec::new();

        if node.x > 0 {
            points.push(Point::from((node.x - 1, node.y)));
        }
        if node.x + 1 < self.max.x {
            points.push(Point::from((node.x + 1, node.y)));
        }
        if node.y > 0 {
            points.push(Point::from((node.x, node.y - 1)));
        }
        if node.y + 1 < self.max.y {
            points.push(Point::from((node.x, node.y + 1)));
        }

        let elevation = *self.elevation.get(&node).unwrap();
        points.retain(|p| {
            let neighbour_elevation = *self.elevation.get(p).unwrap();
            neighbour_elevation < elevation || neighbour_elevation - elevation <= 1
        });

        points
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max.y {
            for x in 0..self.max.x {
                let point = Point::from((x, y));

                if point == self.start {
                    write!(f, "S")?;
                } else if point == self.end {
                    write!(f, "E")?;
                } else {
                    write!(
                        f,
                        "{}",
                        char::from_u32(self.elevation.get(&point).unwrap() + 97).unwrap()
                    )?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let map = Map::from_iter(input.chars());
    println!("{map}");
    println!(
        "{} steps",
        map.find_path(map.start, map.end).unwrap().len() - 1
    );
}
