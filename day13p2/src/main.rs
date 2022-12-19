use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Entry {
    Integer(usize),
    List(Vec<Entry>),
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::from([Vec::new()]);
        let mut buffer = String::new();

        for ch in s.chars() {
            match ch {
                '[' => {
                    stack.push(Vec::new());
                }
                ']' => {
                    let mut list = if let Some(list) = stack.pop() {
                        list
                    } else {
                        panic!("not a list")
                    };

                    if !buffer.is_empty() {
                        list.push(Entry::Integer(usize::from_str(&buffer).unwrap()));
                        buffer = String::new();
                    }

                    if let Some(parent) = stack.last_mut() {
                        parent.push(Entry::List(list));
                    } else {
                        panic!("not a list");
                    }
                }
                ch if ch.is_digit(10) => {
                    buffer.push(ch);
                }
                ',' => {
                    if !buffer.is_empty() {
                        if let Some(parent) = stack.last_mut() {
                            parent.push(Entry::Integer(usize::from_str(&buffer).unwrap()));
                            buffer = String::new();
                        } else {
                            panic!("not a list");
                        }
                    }
                }
                _ => panic!("unexpected char"),
            }
        }

        Ok(stack[0][0].clone())
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Integer(i) => write!(f, "{i}"),
            Entry::List(list) => {
                write!(f, "[{}]", list.iter().join(","))
            }
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (Entry::Integer(int1), Entry::Integer(int2)) => int1.partial_cmp(int2),
            (Entry::List(list1), Entry::List(list2)) => check_order(list1, list2),
            (Entry::List(list1), Entry::Integer(int2)) => {
                let list2 = Vec::from([Entry::Integer(*int2)]);
                check_order(list1, &list2)
            }
            (Entry::Integer(int1), Entry::List(list2)) => {
                let list1 = Vec::from([Entry::Integer(*int1)]);
                check_order(&list1, list2)
            }
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut packets: Vec<Entry> = input
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if !line.is_empty() {
                Some(line.parse().unwrap())
            } else {
                None
            }
        })
        .collect();

    let divider1: Entry = "[[2]]".parse().unwrap();
    let divider2: Entry = "[[6]]".parse().unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_unstable();

    for packet in packets.iter() {
        println!("{packet}");
    }

    let index1 = packets.iter().position(|p| p == &divider1).unwrap() + 1;
    let index2 = packets.iter().position(|p| p == &divider2).unwrap() + 1;
    println!("{index1} * {index2} = {}", index1 * index2);
}

fn check_order(list1: &Vec<Entry>, list2: &Vec<Entry>) -> Option<Ordering> {
    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    loop {
        let (entry1, entry2) = match (iter1.next(), iter2.next()) {
            (None, None) => return Some(Ordering::Equal),
            (None, _) => return Some(Ordering::Less),
            (_, None) => return Some(Ordering::Greater),
            (Some(e1), Some(e2)) => (e1, e2),
        };

        if let Some(ord) = entry1.partial_cmp(entry2) {
            if ord != Ordering::Equal {
                return Some(ord);
            }
        }
    }
}
