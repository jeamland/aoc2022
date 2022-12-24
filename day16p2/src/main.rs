use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::IntoIter;

use itertools::Itertools;

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

#[derive(Clone)]
struct ValvePathIterator<'a> {
    network: &'a ValveNetwork,
    next_steps: Vec<IntoIter<Label>>,
    path: Vec<(Label, usize, usize)>,
    visited: HashSet<Label>,
}

impl<'a> ValvePathIterator<'a> {
    fn new(network: &'a ValveNetwork) -> Self {
        Self {
            network,
            next_steps: Vec::new(),
            path: Vec::new(),
            visited: HashSet::new(),
        }
    }
}

impl<'a> Iterator for ValvePathIterator<'a> {
    type Item = (Vec<Label>, LabelSet, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.path.is_empty() {
            self.path.push((Label::from("AA"), 0, 0));
            self.next_steps
                .push(self.network.viable_labels().into_iter());
        }

        loop {
            let (current_label, current_time, score) = self.path.last()?.clone();
            let candidate = self.next_steps.last_mut()?.next();

            if let Some(candidate) = candidate {
                if self.visited.contains(&candidate) {
                    continue;
                }

                let time = current_time
                    + self
                        .network
                        .path_lengths
                        .get(&(current_label, candidate))
                        .unwrap();

                if time > 26 {
                    continue;
                }

                let score = score + (26 - time) * self.network.valves.get(&candidate).unwrap().rate;
                let mut path: Vec<Label> = self.path.iter().map(|(l, _, _)| *l).collect();
                let mut visited = self.visited.clone();
                path.push(candidate);
                visited.insert(candidate);

                if time < 26 {
                    self.path.push((candidate, time, score));
                    self.visited.insert(candidate);
                    self.next_steps
                        .push(self.network.viable_labels().into_iter());
                }

                return Some((path, LabelSet(visited), score));
            } else {
                let (label, _, _) = self.path.pop().unwrap();
                self.visited.remove(&label);
                self.next_steps.pop();
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LabelSet(HashSet<Label>);

impl LabelSet {
    fn is_disjoint(&self, other: &LabelSet) -> bool {
        self.0.is_disjoint(&other.0)
    }
}

impl Hash for LabelSet {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        let mut labels: Vec<Label> = self.0.iter().cloned().collect();
        labels.sort_unstable();
        for label in labels {
            hasher.write(&(label.0 as u32).to_be_bytes());
            hasher.write(&(label.1 as u32).to_be_bytes());
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let valves = ValveNetwork::from_iter(input.lines().map(|line| line.unwrap()));

    let paths = ValvePathIterator::new(&valves);

    for (path, _, score) in paths.clone() {
        println!("{}: {score}", path.iter().join(" -> "));
    }

    println!();
    println!("{} paths", paths.clone().count());

    let mut best_paths = HashMap::new();
    for (path, visited, score) in paths {
        if let Some((_, existing_score)) = best_paths.get(&visited) {
            if score > *existing_score {
                best_paths.insert(visited, (path, score));
            }
        } else {
            best_paths.insert(visited, (path, score));
        }
    }

    println!("{} paths after pruning", best_paths.len());

    let mut best_pair = None;
    let mut best_score = 0;

    for ((visited1, (path1, score1)), (visited2, (path2, score2))) in
        best_paths.iter().tuple_combinations()
    {
        if !visited1.is_disjoint(&visited2) {
            continue;
        }

        if score1 + score2 > best_score {
            best_score = score1 + score2;
            best_pair = Some((path1, path2));
        }
    }

    if let Some((path1, path2)) = best_pair {
        println!("Participant 1: {}", path1.iter().join(" -> "));
        println!("Participant 2: {}", path2.iter().join(" -> "));
        println!("Score: {}", best_score);
    }
}
