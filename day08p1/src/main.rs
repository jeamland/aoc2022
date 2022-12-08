use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;

trait SingleChar {
    fn single_char(&self) -> char;
}

impl SingleChar for u32 {
    fn single_char(&self) -> char {
        format!("{}", *self).chars().next().unwrap()
    }
}

impl SingleChar for bool {
    fn single_char(&self) -> char {
        if *self {
            '*'
        } else {
            '.'
        }
    }
}

fn mprint<T: SingleChar>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for element in row {
            print!("{}", element.single_char());
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let mut trees = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();

        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trees.push(row);
    }

    mprint(&trees);

    let mut visibility: Vec<Vec<bool>> =
        iter::repeat_with(|| iter::repeat(false).take(trees[0].len()).collect())
            .take(trees.len())
            .collect();

    for row in 0..trees.len() {
        visibility[row][0] = true;
        let mut min = trees[row][0];

        for col in 1..trees[row].len() {
            if trees[row][col] > min {
                visibility[row][col] = true;
                min = trees[row][col]
            }
        }

        visibility[row][trees[row].len() - 1] = true;
        let mut min = trees[row][trees[row].len() - 1];

        for col in (0..trees[row].len() - 1).rev() {
            if trees[row][col] > min {
                visibility[row][col] = true;
                min = trees[row][col]
            }
        }
    }

    for col in 0..trees[0].len() {
        visibility[0][col] = true;
        let mut min = trees[0][col];

        for row in 1..trees.len() {
            if trees[row][col] > min {
                visibility[row][col] = true;
                min = trees[row][col]
            }
        }

        visibility[trees.len() - 1][col] = true;
        let mut min = trees[trees.len() - 1][col];

        for row in (0..trees.len() - 1).rev() {
            if trees[row][col] > min {
                visibility[row][col] = true;
                min = trees[row][col]
            }
        }
    }

    println!();
    mprint(&visibility);

    let visible: usize = visibility
        .iter()
        .map(|row| row.iter().filter(|&v| *v).count())
        .sum();
    println!();
    println!("Total visible: {visible}")
}
