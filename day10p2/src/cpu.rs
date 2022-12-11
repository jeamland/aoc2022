use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy, Debug)]
pub(super) enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    const fn duration(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if let Some(value) = s.strip_prefix("addx ") {
            Ok(Self::AddX(value.parse()?))
        } else {
            Err(anyhow!("unknown instruction"))
        }
    }
}

pub(super) struct Cpu<I: Iterator<Item = Instruction>> {
    instructions: I,
    current_instruction: Option<Instruction>,
    current_completes: usize,

    cycle: usize,
    x: isize,

    display: [bool; 240],
}

impl<I: Iterator<Item = Instruction>> Cpu<I> {
    pub(super) fn new(instructions: I) -> Self {
        Self {
            instructions,
            current_instruction: None,
            current_completes: 0,

            cycle: 0,
            x: 1,

            display: [false; 240],
        }
    }

    pub(super) fn tick(&mut self) -> bool {
        if self.current_instruction.is_none() {
            if let Some(insn) = self.instructions.next() {
                self.current_completes = self.cycle + insn.duration();
                self.current_instruction = Some(insn);
            } else {
                return false;
            }
        }

        let sprite = (self.x - 1)..=(self.x + 1);
        if sprite.contains(&(self.cycle as isize % 40)) {
            self.display[self.cycle] = true;
        }

        self.cycle += 1;

        if self.cycle == self.current_completes {
            match self.current_instruction.unwrap() {
                Instruction::Noop => (),
                Instruction::AddX(value) => self.x += value,
            }

            self.current_instruction = None;
        }

        true
    }

    pub(super) fn cycle(&self) -> usize {
        self.cycle
    }

    pub(super) fn x(&self) -> isize {
        self.x
    }

    pub(super) fn display(&self) -> &[bool] {
        &self.display
    }
}
