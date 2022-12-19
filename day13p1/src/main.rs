use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

#[derive(Clone, Debug)]
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut sum = 0;

    for (index, mut pair) in input.lines().chunks(3).into_iter().enumerate() {
        let (line1, line2) = pair.next_tuple().unwrap();
        let packet1: Entry = line1.unwrap().parse().unwrap();
        let packet2: Entry = line2.unwrap().parse().unwrap();

        let correct = match (&packet1, &packet2) {
            (Entry::List(list1), Entry::List(list2)) => check_order(list1, list2).unwrap(),
            _ => panic!("packets that weren't lists"),
        };

        println!("{packet1}\n{packet2}\n{correct}\n");
        if correct {
            sum += index + 1;
        }
    }

    println!("Sum of correct indices is {sum}");
}

fn check_order(list1: &Vec<Entry>, list2: &Vec<Entry>) -> Option<bool> {
    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    loop {
        let (entry1, entry2) = match (iter1.next(), iter2.next()) {
            (None, None) => return None,
            (None, _) => return Some(true),
            (_, None) => return Some(false),
            (Some(e1), Some(e2)) => (e1, e2),
        };

        match (entry1, entry2) {
            (Entry::Integer(int1), Entry::Integer(int2)) if int1 != int2 => {
                println!("Compare {int1} to {int2}");
                return Some(int1 < int2);
            }
            (Entry::List(list1), Entry::List(list2)) => {
                println!("Compare {list1:?} to {list2:?}");
                if let Some(result) = check_order(list1, list2) {
                    return Some(result);
                }
            }
            (Entry::List(list1), Entry::Integer(int2)) => {
                println!("Compare {list1:?} to {int2}");
                let list2 = Vec::from([Entry::Integer(*int2)]);
                if let Some(result) = check_order(list1, &list2) {
                    return Some(result);
                }
            }
            (Entry::Integer(int1), Entry::List(list2)) => {
                println!("Compare {int1} to {list2:?}");
                let list1 = Vec::from([Entry::Integer(*int1)]);
                if let Some(result) = check_order(&list1, list2) {
                    return Some(result);
                }
            }
            _ => (),
        }
    }
}
