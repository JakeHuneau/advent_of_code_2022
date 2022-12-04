#![allow(unused)]
use crate::{Solution, SolutionPair};
use core::str::FromStr;
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let mut part_one: usize = 0;
    let mut part_two: usize = 0;

    read_to_string("input/day4")
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut split_line = line.split(",");
            let first_section = split_line.next().unwrap().parse::<Section>().unwrap();
            let second_section = split_line.next().unwrap().parse::<Section>().unwrap();
            if first_section.contains(&second_section) || second_section.contains(&first_section) {
                part_one += 1;
                part_two += 1;
            } else if first_section.overlaps(&second_section) {
                part_two += 1;
            }
        });
    (Solution::UInt(part_one), Solution::UInt(part_two))
}

#[derive(Debug, PartialEq)]
struct Section {
    start: usize,
    end: usize,
}

impl FromStr for Section {
    type Err = ();

    /// Section string looks like start-end
    fn from_str(s: &str) -> Result<Section, ()> {
        let mut split_line = s.split("-");
        let start = split_line.next().unwrap().parse::<usize>().unwrap();
        let end = split_line.next().unwrap().parse::<usize>().unwrap();
        Ok(Section { start, end })
    }
}

impl Section {
    /// Checks if this section contains another entirely.
    /// For example, if this section is 2-8, then it contains 3-5.
    fn contains(&self, other_section: &Section) -> bool {
        self.start <= other_section.start && self.end >= other_section.end
    }

    /// Checks if other_section overlaps with this at all.
    /// For example, if this section is 2-8, then it overlaps with 5-10
    fn overlaps(&self, other_section: &Section) -> bool {
        self.start <= other_section.end && self.end >= other_section.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_parse() {
        let test_str = "3-5";
        let test_section = test_str.parse::<Section>().unwrap();
        let expected_section = Section { start: 3, end: 5 };
        assert_eq!(test_section, expected_section);
    }

    #[test]
    fn test_section_contains() {
        let test_section = Section { start: 1, end: 5 };
        let other_section = Section { start: 2, end: 4 };
        assert_eq!(test_section.contains(&other_section), true);
    }

    #[test]
    fn test_section_contains_on_boundaries() {
        let test_section = Section { start: 1, end: 5 };
        assert_eq!(test_section.contains(&test_section), true);
    }

    #[test]
    fn test_section_does_not_contain() {
        let test_section = Section { start: 1, end: 5 };
        let other_section = Section { start: 2, end: 6 };
        assert_eq!(test_section.contains(&other_section), false);
    }

    #[test]
    fn test_overlap_over() {
        let test_section = Section { start: 2, end: 5 };
        let other_section = Section { start: 4, end: 8 };
        assert_eq!(test_section.overlaps(&other_section), true);
    }

    #[test]
    fn test_overlap_under() {
        let test_section = Section { start: 2, end: 5 };
        let other_section = Section { start: 1, end: 3 };
        assert_eq!(test_section.overlaps(&other_section), true);
    }

    #[test]
    fn test_overlap_edge_top() {
        let test_section = Section { start: 2, end: 5 };
        let other_section = Section { start: 5, end: 6 };
        assert_eq!(test_section.overlaps(&other_section), true);
    }

    #[test]
    fn test_overlap_edge_bottom() {
        let test_section = Section { start: 2, end: 5 };
        let other_section = Section { start: 1, end: 2 };
        assert_eq!(test_section.overlaps(&other_section), true);
    }

    #[test]
    fn test_no_overlap_over() {
        let test_section = Section { start: 2, end: 5 };
        let other_section = Section { start: 10, end: 12 };
        assert_eq!(test_section.overlaps(&other_section), false);
    }

    #[test]
    fn test_no_overlap_under() {
        let test_section = Section { start: 3, end: 5 };
        let other_section = Section { start: 1, end: 2 };
        assert_eq!(test_section.overlaps(&other_section), false);
    }
}
