#![allow(unused)]
use crate::{Solution, SolutionPair};
use core::str::FromStr;
use std::collections::LinkedList;
use std::default;
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let mut part_1_stacks: [LinkedList<char>; 9] = Default::default();
    let mut part_2_stacks: [LinkedList<char>; 9] = Default::default();
    let mut still_reading_stacks = true;

    read_to_string("input/day5")
        .unwrap()
        .lines()
        .for_each(|line| {
            if line.starts_with(" 1 ") {
                still_reading_stacks = false;
                part_2_stacks = part_1_stacks.clone();
            }
            if still_reading_stacks {
                parse_stack(line, &mut part_1_stacks);
            } else {
                if line.starts_with("move") {
                    let instruction = line.parse::<Instruction>().unwrap();
                    handle_instruction(&instruction, &mut part_1_stacks, true);
                    handle_instruction(&instruction, &mut part_2_stacks, false);
                }
            }
        });
    (
        Solution::Str(build_stack_str(part_1_stacks)),
        Solution::Str(build_stack_str(part_2_stacks)),
    )
}

/// Parses a string of Stack items into the front of each stack.
/// For example, "[A]                             [I]" will push_front 'A' into stacks[0]
/// and 'I' into stacks[8]
fn parse_stack(line: &str, stacks: &mut [LinkedList<char>; 9]) {
    let chars: Vec<char> = line.chars().collect();
    for i in (1..=33).step_by(4) {
        let this_char = chars[i];
        if this_char != ' ' {
            stacks[i / 4].push_front(this_char);
        }
    }
}

#[derive(PartialEq, Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    /// Parses a string that looks like
    /// move {count} from {from} to {to}
    fn from_str(s: &str) -> Result<Instruction, ()> {
        let words: Vec<&str> = s.split(' ').collect();
        Ok(Instruction {
            count: words[1].parse().unwrap(),
            from: words[3].parse().unwrap(),
            to: words[5].parse().unwrap(),
        })
    }
}

/// Runs the operation from an Instruction onto stacks
/// If not one_at_a_time, then grabs an entire chunk of size instruction.count
/// from the from stack.
fn handle_instruction(
    instruction: &Instruction,
    stacks: &mut [LinkedList<char>; 9],
    one_at_a_time: bool,
) {
    let mut current_stack: LinkedList<char> = Default::default();
    for _ in 0..instruction.count {
        let cargo = &stacks[instruction.from - 1].pop_back().unwrap();
        if one_at_a_time {
            stacks[instruction.to - 1].push_back(*cargo);
        } else {
            current_stack.push_front(*cargo);
        }
    }
    if !one_at_a_time {
        for cargo in current_stack {
            stacks[instruction.to - 1].push_back(cargo);
        }
    }
}

/// Combines back of each LinkedList and combines into string
fn build_stack_str(stacks: [LinkedList<char>; 9]) -> String {
    let mut stack_str = String::from("");
    stacks
        .iter()
        .for_each(|stack| stack_str += stack.back().unwrap().to_string().as_str());
    stack_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stack() {
        let mut stacks: [LinkedList<char>; 9] = Default::default();
        let line = "[A] [B] [C] [D] [E] [F] [G] [H] [I]";
        parse_stack(line, &mut stacks);
        let expected_stacks = [
            LinkedList::from(['A']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_parse_stack_second_stack() {
        let mut stacks: [LinkedList<char>; 9] = [
            LinkedList::from(['A']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        let line = "                                [Z]";
        parse_stack(line, &mut stacks);
        let expected_stacks = [
            LinkedList::from(['A']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['Z', 'I']),
        ];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_parse_stack_empty_space() {
        let mut stacks: [LinkedList<char>; 9] = Default::default();
        let line = "[A]                             [I]";
        parse_stack(line, &mut stacks);
        let expected_stacks = [
            LinkedList::from(['A']),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::from(['I']),
        ];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_parse_instruction() {
        let instruction = "move 1 from 2 to 3".parse::<Instruction>().unwrap();
        let expected_instruction = Instruction {
            count: 1,
            from: 2,
            to: 3,
        };
        assert_eq!(instruction, expected_instruction);
    }

    #[test]
    fn test_instruction() {
        let mut stacks: [LinkedList<char>; 9] = [
            LinkedList::from(['A']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        let instruction = Instruction {
            count: 1,
            from: 1,
            to: 2,
        };
        handle_instruction(&instruction, &mut stacks, true);
        let expected_stacks = [
            LinkedList::from([]),
            LinkedList::from(['B', 'A']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_instruction_multiple() {
        let mut stacks: [LinkedList<char>; 9] = [
            LinkedList::from(['A', 'Z']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        let instruction = Instruction {
            count: 2,
            from: 1,
            to: 2,
        };
        handle_instruction(&instruction, &mut stacks, false);
        let expected_stacks = [
            LinkedList::from([]),
            LinkedList::from(['B', 'A', 'Z']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_build_stack_str() {
        let mut stacks: [LinkedList<char>; 9] = [
            LinkedList::from(['A', 'Z']),
            LinkedList::from(['B']),
            LinkedList::from(['C']),
            LinkedList::from(['D']),
            LinkedList::from(['E']),
            LinkedList::from(['F']),
            LinkedList::from(['G']),
            LinkedList::from(['H']),
            LinkedList::from(['I']),
        ];
        assert_eq!(build_stack_str(stacks), String::from("ZBCDEFGHI"));
    }
}
