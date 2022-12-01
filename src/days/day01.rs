#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let mut fattest_elves: [usize; 3] = [0, 0, 0];
    let mut current_calorie_sum: usize = 0;
    read_to_string("input/day1").unwrap().lines().for_each(|s| {
        if s == "" {
            if fattest_elves.iter().any(|&elf| current_calorie_sum > elf) {
                fattest_elves[0] = current_calorie_sum;
                fattest_elves.sort();
            }
            current_calorie_sum = 0;
        } else {
            current_calorie_sum += s.parse::<usize>().unwrap();
        }
    });
    (
        Solution::UInt(*fattest_elves.iter().max().unwrap()),
        Solution::UInt(fattest_elves.iter().sum::<usize>()),
    )
}
