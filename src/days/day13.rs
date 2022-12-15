#![allow(unused)]
use serde::Deserialize;

use crate::{Solution, SolutionPair};
use std::{cmp::Ordering, fs::read_to_string};

pub fn solve() -> SolutionPair {
    let mut part_1 = 0;
    let mut all_packets = vec![
        Packet::List(vec![Packet::Int(2)]),
        Packet::List(vec![Packet::Int(6)]),
    ];
    read_to_string("input/day13")
        .expect("Couldn't read data")
        .split("\n\n")
        .enumerate()
        .for_each(|(index, packet_pair)| {
            let mut packet_split = packet_pair.lines();
            let packet_1 = serde_json::from_str::<Packet>(packet_split.next().unwrap()).unwrap();
            let packet_2 = serde_json::from_str::<Packet>(packet_split.next().unwrap()).unwrap();
            all_packets.push(packet_1.clone());
            all_packets.push(packet_2.clone());

            if packet_1 < packet_2 {
                part_1 += index + 1;
            }
        });

    all_packets.sort();
    let packet_1_location = all_packets
        .binary_search(&Packet::List(vec![Packet::Int(2)]))
        .unwrap()
        + 1;
    let packet_2_location = all_packets
        .binary_search(&Packet::List(vec![Packet::Int(6)]))
        .unwrap()
        + 1;
    (
        Solution::UInt(part_1),
        Solution::UInt(packet_1_location * packet_2_location),
    )
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(left), Packet::Int(right)) => left.cmp(right),
            (Packet::Int(left), Packet::List(right)) => [Packet::Int(*left)][..].cmp(right),
            (Packet::List(left), Packet::Int(right)) => left.as_slice().cmp(&[Packet::Int(*right)]),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
        }
    }
}
