use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::File, io::Read};

pub fn solve() -> SolutionPair {
    let mut input = File::open("input/day6").expect("Could not open file");
    let mut buffer = [0];
    let mut window_1: [u8; 4] = [0; 4];
    let mut window_2: [u8; 14] = [0; 14];
    let mut count = 0;
    let mut part_1 = 0;
    let mut part_2 = 0;

    loop {
        match input.read_exact(&mut buffer) {
            Ok(()) => {
                count += 1;
                window_1[3] = buffer[0];
                window_2[13] = buffer[0];
                window_1.rotate_right(1);
                window_2.rotate_right(1);
                if part_1 == 0 && count >= 4 && HashSet::from(window_1).len() == 4 {
                    part_1 = count;
                }
                if count >= 14 && HashSet::from(window_2).len() == 14 {
                    part_2 = count;
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    (Solution::UInt(part_1), Solution::UInt(part_2))
}
