use std::{cell::RefCell, cmp::Reverse, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Monkey {
    items: RefCell<Vec<u64>>,
    operator: String,
    operand: String,
    test: u64,
    truthy: usize,
    falsy: usize,
    operations: u64,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info: Vec<&str> = s.lines().map(str::trim).collect();

        Ok(Self {
            items: RefCell::new(
                info[1][16..]
                    .split(", ")
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect(),
            ),
            operator: info[2].chars().nth(21).unwrap().to_string(),
            operand: info[2][23..].to_string(),
            test: info[3][19..].parse().unwrap(),
            truthy: info[4][25..].parse().unwrap(),
            falsy: info[5][26..].parse().unwrap(),
            operations: 0,
        })
    }
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(usize, u64)> {
        let mut items = self.items.borrow_mut();
        let mut throw: Vec<(usize, u64)> = vec![];

        for item in items.iter() {
            let mut i = *item;
            let operand = match self.operand.as_str() {
                "old" => i,
                _ => self.operand.parse().unwrap(),
            };

            match self.operator.as_str() {
                "+" => i += operand,
                "*" => i *= operand,
                _ => unreachable!(),
            }

            i /= 3;

            self.operations += 1;

            if i % self.test == 0 {
                throw.push((self.truthy, i));
            } else {
                throw.push((self.falsy, i));
            }
        }

        items.clear();
        throw
    }

    fn push(&self, item: u64) {
        let mut items = self.items.borrow_mut();
        items.push(item);
    }
}

fn part_one() -> u64 {
    let mut monkeys: Vec<Monkey> = include_str!("input")
        .split("\n\n")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            for (monkey, item) in monkeys[monkey].inspect() {
                monkeys[monkey].push(item);
            }
        }
    }

    let mut operations: Vec<u64> = monkeys.iter().map(|monkey| monkey.operations).collect();
    operations.sort_by_key(|w| Reverse(*w));
    operations.iter().take(2).product()
}

fn part_two() -> u64 {
    let mut monkeys: Vec<Monkey> = include_str!("example")
        .split("\n\n")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            for (monkey, item) in monkeys[monkey].inspect() {
                monkeys[monkey].push(item);
            }
        }
    }

    let mut operations: Vec<u64> = monkeys.iter().map(|monkey| monkey.operations).collect();
    operations.sort_by_key(|w| Reverse(*w));
    operations.iter().take(2).product()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
