#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let mut grid = read_to_string("input/day12")
        .expect("Couldn't read file")
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_location = find_value(&grid, b'S').unwrap();
    let end_location = find_value(&grid, b'E').unwrap();

    // Need to make the start and end the elevation
    grid[start_location.x][start_location.y] = b'a';
    grid[end_location.x][end_location.y] = b'z';

    let sol_1 = bfs(&grid, start_location, &end_location).unwrap();

    let mut sol_2: usize = 1000;
    let mut low_points: Vec<Location> = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == b'a' {
                if let Some(distance) = bfs(&grid, Location { x, y }, &end_location) {
                    if distance < sol_2 {
                        sol_2 = distance;
                    }
                }
            }
        }
    }
    (Solution::UInt(sol_1), Solution::UInt(sol_2))
}

fn bfs(grid: &Vec<Vec<u8>>, start: Location, goal: &Location) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((location, steps)) = queue.pop_front() {
        if *goal == location {
            return Some(steps);
        }
        for (x_move, y_move) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (x_test, y_test) = (
                (location.x as isize + x_move) as usize,
                (location.y as isize + y_move) as usize,
            );
            let Some(&location_test) = grid.get(x_test).and_then(|row| row.get(y_test)) else { continue };
            if grid[location.x][location.y] + 1 >= location_test && !visited[x_test][y_test] {
                visited[x_test][y_test] = true;
                queue.push_back((
                    Location {
                        x: x_test,
                        y: y_test,
                    },
                    steps + 1,
                ));
            }
        }
    }
    None
}

#[derive(Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

fn find_value(grid: &Vec<Vec<u8>>, value: u8) -> Option<Location> {
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == value {
                return Some(Location { x, y });
            }
        }
    }
    None
}
