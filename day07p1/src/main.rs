use std::collections::btree_map::{BTreeMap, Iter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Entry {
    Directory(BTreeMap<String, Entry>),
    File(usize),
}

impl Entry {
    fn lookup_dir_mut(&mut self, path: &[String]) -> &mut BTreeMap<String, Entry> {
        if let Self::Directory(entries) = self {
            if path.is_empty() {
                entries
            } else {
                entries
                    .get_mut(&path[0])
                    .unwrap()
                    .lookup_dir_mut(&path[1..])
            }
        } else {
            panic!("not a dir");
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Directory(entries) => entries.values().map(|e| e.size()).sum(),
        }
    }

    fn iter<'a>(&'a self) -> EntryIterator<'a> {
        if let Self::Directory(_) = self {
            EntryIterator::new(self)
        } else {
            panic!("not a dir");
        }
    }
}

struct EntryIterator<'a> {
    root: &'a Entry,
    stack: Vec<(String, Iter<'a, String, Entry>)>,
}

impl<'a> EntryIterator<'a> {
    fn new(root: &'a Entry) -> Self {
        Self {
            root,
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for EntryIterator<'a> {
    type Item = (String, &'a Entry);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            if let Entry::Directory(entries) = self.root {
                self.stack.push((String::new(), entries.iter()));
                return Some(("/".into(), self.root));
            } else {
                panic!("not a dir");
            }
        }

        loop {
            let (path, iter) = self.stack.last_mut()?;

            if let Some((name, entry)) = iter.next() {
                let mut path = path.clone();
                path.push_str("/");
                path.push_str(name);

                if let Entry::Directory(entries) = entry {
                    self.stack.push((path.clone(), entries.iter()));
                }

                return Some((path, entry));
            } else {
                self.stack.pop();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut root = Entry::Directory(BTreeMap::new());
    let mut path_stack = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();

        if let Some(path) = line.strip_prefix("$ cd ") {
            match path {
                "/" => {
                    path_stack = Vec::new();
                }
                ".." => {
                    path_stack.pop();
                }
                _ => {
                    path_stack.push(path.to_owned());
                }
            }
        } else if !line.starts_with("$ ") {
            let (desc, name) = line.split_once(' ').unwrap();
            let entry = if desc == "dir" {
                Entry::Directory(BTreeMap::new())
            } else {
                Entry::File(desc.parse().unwrap())
            };

            let cwd = root.lookup_dir_mut(&path_stack);
            cwd.insert(name.to_owned(), entry);
        }
    }

    let mut total = 0;

    for (path, entry) in root.iter() {
        if matches!(entry, Entry::Directory(_)) {
            let size = entry.size();

            println!("{path:60} - {:10}", size);

            if size <= 100000 {
                total += size;
            }
        }
    }

    println!("Total size of selected dirs: {total}")
}
