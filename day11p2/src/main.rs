use itertools::Itertools;

#[derive(Debug)]
struct Item(usize);

#[derive(Debug)]
enum Operand {
    Old,
    Number(usize),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        if s == "old" {
            Self::Old
        } else {
            Self::Number(s.parse().unwrap())
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("unknown operator"),
        }
    }
}

#[derive(Debug)]
struct Operation {
    op: Operator,
    arg: Operand,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let (op, arg) = s.split_once(' ').unwrap();
        let op = Operator::from(op);
        let arg = Operand::from(arg);

        Self { op, arg }
    }
}

impl Operation {
    fn perform(&self, item: &mut Item, modulus: usize) {
        let arg = match self.arg {
            Operand::Old => item.0,
            Operand::Number(n) => n,
        };

        item.0 = match self.op {
            Operator::Add => item.0 + arg,
            Operator::Multiply => item.0 * arg,
        } % modulus;
    }
}

#[derive(Debug)]
struct TargetTest {
    divisor: usize,
    true_target: usize,
    false_target: usize,
}

impl TargetTest {
    fn choose(&self, item: &Item) -> usize {
        if item.0 % self.divisor == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    target: TargetTest,

    inspections: usize,
}

impl Monkey {
    fn throws(&mut self, modulus: usize) -> Vec<(Item, usize)> {
        self.items
            .drain(0..)
            .map(|mut item| {
                self.operation.perform(&mut item, modulus);
                self.inspections += 1;
                let target = self.target.choose(&item);
                (item, target)
            })
            .collect()
    }

    fn catch(&mut self, item: Item) {
        self.items.push(item)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let mut monkeys = Vec::new();
    let mut modulus = 1;

    for monkey in input.split_terminator("\n\n") {
        let lines: Vec<&str> = monkey.split_terminator('\n').collect();

        let (_, items) = lines[1].rsplit_once(": ").unwrap();
        let items: Vec<Item> = items
            .split(", ")
            .map(|v| Item(v.parse().unwrap()))
            .collect();

        let (_, operation) = lines[2].split_once(" = old ").unwrap();
        let operation = Operation::from(operation);

        let (_, divisor) = lines[3].rsplit_once(' ').unwrap();
        let divisor: usize = divisor.parse().unwrap();
        modulus *= divisor;

        let (_, true_target) = lines[4].rsplit_once(' ').unwrap();
        let true_target: usize = true_target.parse().unwrap();

        let (_, false_target) = lines[5].rsplit_once(' ').unwrap();
        let false_target: usize = false_target.parse().unwrap();

        let target = TargetTest {
            divisor,
            true_target,
            false_target,
        };

        monkeys.push(Monkey {
            items,
            operation,
            target,
            inspections: 0,
        });
    }

    for round in 1..=10000 {
        for monkey in 0..monkeys.len() {
            for (item, target) in monkeys[monkey].throws(modulus) {
                monkeys[target].catch(item);
            }
        }

        println!("After round {round}, the monkeys are holding items with these worry levels:");
        for monkey in 0..monkeys.len() {
            println!(
                "Monkey {monkey}: {}",
                monkeys[monkey]
                    .items
                    .iter()
                    .map(|i| format!("{}", i.0))
                    .join(", ")
            );
        }
    }

    println!();

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i} inspected items {} times.", monkey.inspections);
    }

    monkeys.sort_unstable_by_key(|m| m.inspections);
    monkeys.reverse();

    println!();

    println!(
        "Monkey business: {}",
        monkeys[0].inspections * monkeys[1].inspections
    );
}
