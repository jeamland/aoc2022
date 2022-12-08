use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn mprint(matrix: &Vec<Vec<u32>>) {
    for row in matrix {
        for element in row {
            print!("{element}");
        }
        println!();
    }
    println!();
}

fn score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> usize {
    let mut up = 0;
    for r in (0..row).rev() {
        up += 1;
        if trees[r][col] >= trees[row][col] {
            break;
        }
    }

    let mut down = 0;
    for r in (row + 1)..trees.len() {
        down += 1;
        if trees[r][col] >= trees[row][col] {
            break;
        }
    }

    let mut left = 0;
    for c in (0..col).rev() {
        left += 1;
        if trees[row][c] >= trees[row][col] {
            break;
        }
    }

    let mut right = 0;
    for c in (col + 1)..trees[col].len() {
        right += 1;
        if trees[row][c] >= trees[row][col] {
            break;
        }
    }

    println!(
        "({row}, {col}) {up} {left} {down} {right} -> {}",
        up * down * left * right
    );
    up * down * left * right
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

    let mut scores = Vec::new();

    for row in 0..trees.len() {
        let mut score_row = Vec::new();

        for col in 0..trees[row].len() {
            score_row.push(score(row, col, &trees));
        }

        scores.push(score_row);
    }

    println!();

    let max_score = scores
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Best score is {max_score}");
}
