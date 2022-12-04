#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

/// Gets the value of a char where a = 1, b = 2, ... z = 26, A = 27, ..., Z = 53
fn get_value(ch: char) -> usize {
    let char_val = ch as usize;
    match ch.is_uppercase() {
        true => char_val + 27 - 'A' as usize,
        false => char_val + 1 - 'a' as usize,
    }
}

pub fn solve() -> SolutionPair {
    let part_1 = read_to_string("input/day3")
        .unwrap()
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let half_len = chars.len() / 2;
            let first_half: HashSet<&char> = HashSet::from_iter(&chars[..half_len]);
            let second_half: HashSet<&char> = HashSet::from_iter(&chars[half_len..]);
            let intersection = first_half.intersection(&second_half);
            intersection.map(|&ch| get_value(*ch)).sum::<usize>()
        })
        .sum();

    let part_2 = read_to_string("input/day3")
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|line| {
            let first_line: HashSet<char> = HashSet::from_iter(line[0].chars());
            let second_line: HashSet<char> = HashSet::from_iter(line[1].chars());
            let third_line: HashSet<char> = HashSet::from_iter(line[2].chars());
            first_line
                .iter()
                .filter(|ch| second_line.contains(ch) && third_line.contains(ch))
                .map(|&ch| get_value(ch))
                .sum::<usize>()
        })
        .sum::<usize>();
    (Solution::UInt(part_1), Solution::UInt(part_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_lowercase() {
        assert_eq!(get_value('a'), 1);
        assert_eq!(get_value('z'), 26);
    }

    #[test]
    fn test_get_value_uppercase() {
        assert_eq!(get_value('A'), 27);
        assert_eq!(get_value('Z'), 52);
    }
}
