#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{
    collections::{HashSet, LinkedList},
    fs::read_to_string,
    ops::{AddAssign, Sub},
};

pub fn solve() -> SolutionPair {
    let mut head_location = Location { x: 0, y: 0 };
    let mut rope: [RopePiece; 9] = Default::default();
    read_to_string("input/day9")
        .expect("Could not read input")
        .lines()
        .for_each(|line| {
            let mut line_split = line.split_whitespace();
            let movement = get_movement(line_split.next().expect("Could not get distance"));
            let distance = line_split
                .next()
                .expect("Could not get distance")
                .parse::<usize>()
                .expect("Could not parse distance");
            for _ in 0..distance {
                head_location += movement.clone();
                rope[8].follow(&head_location);
                for i in (0..8).rev() {
                    rope[i].follow(&rope[i + 1].current_position.clone());
                }
            }
        });
    (
        Solution::UInt(rope[8].past_locations.len()),
        Solution::UInt(rope[0].past_locations.len()),
    )
}

fn get_movement(direction: &str) -> Location {
    match direction {
        "R" => Location { x: 1, y: 0 },
        "L" => Location { x: -1, y: 0 },
        "U" => Location { x: 0, y: 1 },
        "D" => Location { x: 0, y: -1 },
        _ => Location { x: 0, y: 0 },
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Location {
    x: isize,
    y: isize,
}

impl AddAssign for Location {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Location {
    fn is_next_to(&self, position: &Location) -> bool {
        self.x.abs_diff(position.x) <= 1 && self.y.abs_diff(position.y) <= 1
    }
}

struct RopePiece {
    current_position: Location,
    past_locations: HashSet<Location>,
}

impl Default for RopePiece {
    fn default() -> Self {
        Self {
            current_position: Location { x: 0, y: 0 },
            past_locations: HashSet::from_iter(vec![Location { x: 0, y: 0 }]),
        }
    }
}

impl RopePiece {
    fn follow(&mut self, head: &Location) {
        if !self.current_position.is_next_to(head) {
            let diff = head.clone() - self.current_position.clone();

            self.current_position.x += diff.x.signum();
            self.current_position.y += diff.y.signum();

            self.past_locations.insert(self.current_position.clone());
        }
    }
}
