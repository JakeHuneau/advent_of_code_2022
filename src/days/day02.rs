#![allow(unused)]
use crate::{Solution, SolutionPair};
use core::str::FromStr;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Hand {
    type Err = ();

    /// For parsing a string in part one
    fn from_str(s: &str) -> Result<Hand, ()> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(()),
        }
    }
}

impl Hand {
    /// Figures out what player_2 will have based on an outcome
    fn get_opponent(&self, outcome: Outcome) -> Hand {
        match self {
            Hand::Rock => match outcome {
                Outcome::Lose => Hand::Scissor,
                Outcome::Draw => Hand::Rock,
                Outcome::Win => Hand::Paper,
            },
            Hand::Paper => match outcome {
                Outcome::Lose => Hand::Rock,
                Outcome::Draw => Hand::Paper,
                Outcome::Win => Hand::Scissor,
            },
            Hand::Scissor => match outcome {
                Outcome::Lose => Hand::Paper,
                Outcome::Draw => Hand::Scissor,
                Outcome::Win => Hand::Rock,
            },
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ();

    /// For part 2
    fn from_str(s: &str) -> Result<Outcome, ()> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

struct Battle {
    player_1: Hand,
    player_2: Hand,
}

impl FromStr for Battle {
    type Err = ();

    /// For part one. Takes a str like "A Y" and converts that to Battle {Rock, Paper}
    fn from_str(s: &str) -> Result<Battle, ()> {
        let mut split_line = s.split(" ");
        let player_1 = split_line.next().unwrap().parse::<Hand>().unwrap();
        let player_2 = split_line.next().unwrap().parse::<Hand>().unwrap();
        Ok(Battle { player_1, player_2 })
    }
}

impl Battle {
    fn battle(&self) -> Outcome {
        // Outcome for player_2
        if self.player_1 == self.player_2 {
            return Outcome::Draw;
        }
        if self.player_2 == Hand::Rock {
            if self.player_1 == Hand::Paper {
                return Outcome::Lose;
            }
            return Outcome::Win;
        }
        if self.player_2 == Hand::Paper {
            if self.player_1 == Hand::Scissor {
                return Outcome::Lose;
            }
            return Outcome::Win;
        }
        // player_2 must be Hand::Scissor
        if self.player_1 == Hand::Rock {
            return Outcome::Lose;
        }
        Outcome::Win
    }

    fn score(&self) -> usize {
        let hand_score: usize = match self.player_2 {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        };
        let outcome_score: usize = match self.battle() {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        hand_score + outcome_score
    }

    /// We know player_1 from the str, then calculate player_2 based on the outcome
    fn from_outcome(line: &str) -> Battle {
        let mut split = line.split(" ");
        let player_1 = split.next().unwrap().parse::<Hand>().unwrap();
        let player_2 = player_1.get_opponent(split.next().unwrap().parse::<Outcome>().unwrap());
        Battle { player_1, player_2 }
    }
}

pub fn solve() -> SolutionPair {
    // Both parts parse each line into a Battle then calculate scores and sum

    let part_1_score = read_to_string("input/day2")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Battle>().unwrap().score())
        .sum();

    let part_2_score: usize = read_to_string("input/day2")
        .unwrap()
        .lines()
        .map(|line| Battle::from_outcome(line).score())
        .sum();

    (Solution::UInt(part_1_score), Solution::UInt(part_2_score))
}
