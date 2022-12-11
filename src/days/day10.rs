#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, str::FromStr};

pub fn solve() -> SolutionPair {
    let mut current_value: isize = 1;
    let mut current_tick: isize = 0;
    let mut sum: isize = 0;

    let mut hit_benchmarks = [false, false, false, false, false, false];
    let mut screen: Vec<Vec<char>> = vec![vec![' '; 40]; 6];

    read_to_string("input/day10")
        .expect("Could not read file")
        .lines()
        .for_each(|line| {
            line.parse::<Operation>()
                .expect("Could not parse line")
                .run(&mut current_value, &mut current_tick, &mut sum, &mut screen);
        });
    for i in &screen {
        println!("{:?}", i.iter().collect::<String>());
    }
    (Solution::Int(sum), Solution::UInt(1))
}

fn tick(
    tick_count: &mut isize,
    current_value: &isize,
    sum: &mut isize,
    screen: &mut Vec<Vec<char>>,
) {
    if (*tick_count % 40).abs_diff(*current_value) <= 1 {
        screen[*tick_count as usize / 40][(*tick_count % 40) as usize] = '█';
    }

    *tick_count += 1;
    if [20, 60, 100, 140, 180, 220].contains(&tick_count) {
        *sum += *current_value * *tick_count;
    }
}

enum Operation {
    Noop,
    Addx(isize),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<&str>>();
        match split.len() {
            1 => Ok(Self::Noop),
            2 => Ok(Self::Addx(
                split[1].parse::<isize>().expect("Error parsing"),
            )),
            _ => Err(()),
        }
    }
}

impl Operation {
    fn run(
        &self,
        current_value: &mut isize,
        tick_count: &mut isize,
        sum: &mut isize,
        screen: &mut Vec<Vec<char>>,
    ) {
        match self {
            Self::Noop => {
                tick(tick_count, current_value, sum, screen);
            }
            Self::Addx(value) => {
                tick(tick_count, current_value, sum, screen);
                tick(tick_count, current_value, sum, screen);
                *current_value += value;
            }
        }
    }
}
