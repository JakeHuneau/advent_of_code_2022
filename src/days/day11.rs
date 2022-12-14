#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{collections::LinkedList, fs::read_to_string, str::FromStr};

pub fn solve() -> SolutionPair {
    let mut monkeys = read_to_string("input/day11")
        .expect("Couldn't read input")
        .split("\n\n")
        .map(|part| part.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();
    let mut monkeys_part_2 = monkeys.clone();
    let modulo: usize = monkeys_part_2
        .iter()
        .map(|monkey| monkey.divisible_test)
        .product();

    for _ in 0..20 {
        do_round(&mut monkeys, true, 0);
    }
    let mut monkey_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();
    monkey_counts.sort();
    monkey_counts.reverse();
    let sol_1 = monkey_counts[0] * monkey_counts[1];

    for _ in 0..10000 {
        do_round(&mut monkeys_part_2, false, modulo);
    }
    let mut monkey_counts = monkeys_part_2
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();
    monkey_counts.sort();
    monkey_counts.reverse();
    let sol_2 = monkey_counts[0] * monkey_counts[1];

    (Solution::UInt(sol_1), Solution::UInt(sol_2))
}

fn do_round(monkeys: &mut Vec<Monkey>, part_1: bool, modulo: usize) {
    for i in 0..monkeys.len() {
        for _ in 0..monkeys[i].items.len() {
            let (item, monkey_index) = monkeys[i].inspect(part_1, modulo);
            monkeys[monkey_index].items.push_back(item);
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: LinkedList<usize>,
    operation: Operation,
    operation_value: usize,
    divisible_test: usize,
    true_throw: usize,
    false_throw: usize,
    inspections: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().unwrap(); // Monkey x

        // Starting items: x, y
        let items = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect::<LinkedList<usize>>();

        // Operation: new = old +/* x
        let operation_line = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let operation = match operation_line[4] == "*" {
            true => Operation::Multiply,
            false => Operation::Add,
        };
        let operation_value = match operation_line[5] == "old" {
            true => 0,
            false => operation_line[5].parse::<usize>().unwrap(),
        };

        // Test: divisible by 23
        let divisible_test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[3]
            .parse::<usize>()
            .unwrap();

        // If true: throw to monkey 2
        let true_throw = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .unwrap();

        // If false: throw to monkey 3
        let false_throw = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[5]
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            items,
            operation,
            operation_value,
            divisible_test,
            true_throw,
            false_throw,
            inspections: 0,
        })
    }
}

impl Monkey {
    fn inspect(&mut self, part_1: bool, modulo: usize) -> (usize, usize) {
        self.inspections += 1;
        let mut inspection_item = self.items.pop_front().unwrap();
        match self.operation {
            Operation::Multiply => match self.operation_value {
                0 => inspection_item *= inspection_item,
                (value) => inspection_item *= value,
            },
            Operation::Add => match self.operation_value {
                0 => inspection_item += inspection_item,
                (value) => inspection_item += value,
            },
        };
        if part_1 {
            inspection_item /= 3;
        } else {
            inspection_item %= modulo;
        }
        let throw_to = match inspection_item % self.divisible_test == 0 {
            true => self.true_throw,
            false => self.false_throw,
        };
        (inspection_item, throw_to)
    }
}
