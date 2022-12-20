#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string, str::FromStr};

pub fn solve() -> SolutionPair {
    let sensor_beacons = read_to_string("input/day15")
        .unwrap()
        .lines()
        .map(|line| line.parse::<SensorBeacon>().unwrap())
        .collect::<Vec<_>>();
    let mut sol_1_locations: HashSet<(isize, isize)> = HashSet::new();
    let sol_1 = sensor_beacons.iter().for_each(|sb| {
        sol_1_locations.extend(sb.covered_spaces(10));
    });

    let sol_2 = part_2(sensor_beacons, (0, 0), (4000000, 4000000)).unwrap();

    (
        Solution::UInt(sol_1_locations.len()),
        Solution::Int(4000000 * sol_2.0 + sol_2.1),
    )
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: (isize, isize),
    beacon: (isize, isize),
    distance: usize,
}

impl FromStr for SensorBeacon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().collect::<Vec<_>>();
        let sensor_x_raw = split[2].chars().collect::<Vec<_>>();
        let sensor_x = sensor_x_raw[2..sensor_x_raw.len() - 1]
            .iter()
            .collect::<String>()
            .as_str()
            .parse::<isize>()
            .unwrap();

        let sensor_y_raw = split[3].chars().collect::<Vec<_>>();
        let sensor_y = sensor_y_raw[2..sensor_y_raw.len() - 1]
            .iter()
            .collect::<String>()
            .as_str()
            .parse::<isize>()
            .unwrap();

        let beacon_x_raw = split[8].chars().collect::<Vec<_>>();
        let beacon_x = beacon_x_raw[2..beacon_x_raw.len() - 1]
            .iter()
            .collect::<String>()
            .as_str()
            .parse::<isize>()
            .unwrap();
        let beacon_y_raw = split[9].chars().collect::<Vec<_>>();
        let beacon_y = beacon_y_raw[2..beacon_y_raw.len()]
            .iter()
            .collect::<String>()
            .as_str()
            .parse::<isize>()
            .unwrap();

        Ok(Self {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
            distance: sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y),
        })
    }
}

impl SensorBeacon {
    fn covered_spaces(&self, row: usize) -> HashSet<(isize, isize)> {
        let mut this_set = HashSet::<(isize, isize)>::new();
        if self.sensor.1.abs_diff(row.try_into().unwrap()) > self.distance {
            return this_set;
        }

        let width = (self
            .distance
            .abs_diff(self.sensor.1.abs_diff(row.try_into().unwrap())))
            as isize;
        for x in self.sensor.0 - width..self.sensor.0 + width {
            this_set.insert((x, row.try_into().unwrap()));
        }
        this_set
    }

    fn contains(&self, other: (isize, isize)) -> bool {
        self.sensor.0.abs_diff(other.0) + self.sensor.1.abs_diff(other.1) <= self.distance
    }

    fn can_contain(&self, min: (isize, isize), max: (isize, isize)) -> bool {
        let corners = [
            (min.0, min.1),
            (min.0, max.1),
            (max.0, min.1),
            (max.0, max.1),
        ];
        let largest_dist = corners
            .iter()
            .map(|corner| {
                (self.sensor.0).abs_diff(corner.0.try_into().unwrap())
                    + self.sensor.1.abs_diff(corner.1.try_into().unwrap())
            })
            .max()
            .unwrap();
        largest_dist > self.distance
    }
}

/// Divide and conquer
fn part_2(
    sensor_beacons: Vec<SensorBeacon>,
    min: (isize, isize),
    max: (isize, isize),
) -> Option<(isize, isize)> {
    let mut quad_stack = vec![(min, max)];

    while let Some((min, max)) = quad_stack.pop() {
        if min == max {
            if sensor_beacons.iter().all(|sb| !sb.contains(min)) {
                return Some(min);
            }
        } else {
            let mid = ((min.0 + max.0) / 2, (min.1 + max.1) / 2);
            let quadrants = [
                (min, mid),
                ((mid.0 + 1, min.1), (max.0, mid.1)),
                ((min.0, mid.1 + 1), (mid.0, max.1)),
                ((mid.0 + 1, mid.1 + 1), max),
            ];
            for quad in quadrants.iter() {
                if quad.0 .0 > quad.1 .0 || quad.0 .1 > quad.1 .1 {
                    continue;
                }

                if sensor_beacons
                    .iter()
                    .all(|sb| sb.can_contain(quad.0, quad.1))
                {
                    quad_stack.push(*quad);
                }
            }
        }
    }
    None
}
