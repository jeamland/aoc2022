use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;
use priority_queue::PriorityQueue;

mod astar;
use astar::AStar;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Label(char, char);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Label({}{})", self.0, self.1)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl From<&str> for Label {
    fn from(s: &str) -> Label {
        Self(s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap())
    }
}

#[derive(Debug)]
struct Valve {
    rate: usize,
    connections: Vec<Label>,
}

impl Valve {
    fn new(rate: usize, connections: Vec<Label>) -> Self {
        Self { rate, connections }
    }
}

#[derive(Debug, Default)]
struct ValveNetwork {
    valves: HashMap<Label, Valve>,
    path_lengths: HashMap<(Label, Label), usize>,
}

impl ValveNetwork {
    fn viable_labels(&self) -> Vec<Label> {
        self.valves
            .iter()
            .filter_map(|(label, valve)| {
                if valve.rate > 0 {
                    Some(label.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl AStar for ValveNetwork {
    type Node = Label;
    type Weight = usize;

    fn heuristic(&self, _from: &Label, _to: &Label) -> usize {
        self.valves.len()
    }

    fn weight(&self, _from: &Label, _to: &Label) -> usize {
        1
    }

    fn neighbours(&self, node: &Label) -> Vec<Label> {
        self.valves.get(&node).unwrap().connections.clone()
    }
}

impl FromIterator<String> for ValveNetwork {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let mut network = Self::default();

        for line in iter {
            let label: Label = (&line[6..8]).into();

            let semicolon = line.chars().position(|ch| ch == ';').unwrap();
            let rate: usize = (&line[23..semicolon]).parse().unwrap();

            let connections = if let Some((_, c)) = line.split_once("valves ") {
                c
            } else if let Some((_, c)) = line.split_once("valve ") {
                c
            } else {
                panic!("malformed connection list");
            };
            let connections: Vec<Label> = connections.split(", ").map(|c| c.into()).collect();

            network.valves.insert(label, Valve::new(rate, connections));
        }

        let labels = network.viable_labels();
        let aa = Label::from("AA");

        for label in &labels {
            let path = network.find_path(&aa, label).unwrap();
            network.path_lengths.insert((aa, *label), path.len());
        }

        for pair in labels.iter().combinations(2) {
            let path = network.find_path(pair[0], pair[1]).unwrap();
            network
                .path_lengths
                .insert((*pair[0], *pair[1]), path.len());
            network
                .path_lengths
                .insert((*pair[1], *pair[0]), path.len());
        }

        network
    }
}

struct ValvePlanner {
    valves: ValveNetwork,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PlannerNode {
    current_time: usize,
    current_label: Label,
    open_valves: Vec<(Label, usize)>,
}

impl PlannerNode {
    fn open_valves(&self) -> Vec<Label> {
        self.open_valves.iter().map(|(l, _)| *l).collect()
    }
}

impl ValvePlanner {
    fn new(valves: ValveNetwork) -> Self {
        Self { valves }
    }

    fn score(&self, node: &PlannerNode) -> usize {
        let mut flow = 0;

        for (label, open_time) in &node.open_valves {
            flow += (30 - open_time) * self.valves.valves.get(&label).unwrap().rate;
        }

        flow
    }

    fn neighbours(&self, node: &PlannerNode) -> Vec<PlannerNode> {
        let mut labels: HashSet<Label> = HashSet::from_iter(self.valves.viable_labels());
        for label in node.open_valves() {
            labels.remove(&label);
        }

        let mut neighbours = Vec::new();

        for label in labels {
            let distance = self
                .valves
                .path_lengths
                .get(&(node.current_label, label))
                .unwrap();
            let time = node.current_time + distance;
            if time > 30 {
                continue;
            }

            let mut open_valves = node.open_valves.clone();
            open_valves.push((label, time));

            neighbours.push(PlannerNode {
                current_time: time,
                current_label: label,
                open_valves,
            })
        }

        neighbours
    }

    fn find_plan(&self) -> PlannerNode {
        let mut open_set = PriorityQueue::new();

        let start_node = PlannerNode {
            current_time: 0,
            current_label: Label::from("AA"),
            open_valves: Vec::new(),
        };
        open_set.push(start_node.clone(), self.score(&start_node));
        let mut best_node = start_node.clone();

        while let Some((current, score)) = open_set.pop() {
            if score > self.score(&best_node) {
                best_node = current.clone();
            }

            for neighbour in self.neighbours(&current) {
                open_set.push_increase(neighbour.clone(), self.score(&neighbour));
            }
        }

        best_node
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let valves = ValveNetwork::from_iter(input.lines().map(|line| line.unwrap()));
    let planner = ValvePlanner::new(valves);
    let node = planner.find_plan();

    let path = node
        .open_valves
        .iter()
        .map(|(label, time)| format!("{label} @ {time}"))
        .join(" -> ");
    let score = planner.score(&node);

    println!("{path}");
    println!("Total released: {score}");
}
