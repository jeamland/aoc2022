use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod cpu;

use cpu::Cpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = BufReader::new(File::open(&args[1]).unwrap());

    let instructions = input.lines().map(|l| l.unwrap().parse().unwrap());

    let mut cpu = Cpu::new(instructions);

    while cpu.cycle() < 240 {
        if !cpu.tick() {
            break;
        }

        println!("X={:4} at cycle {:3}", cpu.x(), cpu.cycle());
    }

    for (i, pixel) in cpu.display().iter().enumerate() {
        print!("{}", if *pixel { '#' } else { '.' });
        if i % 40 == 39 {
            println!();
        }
    }
}
