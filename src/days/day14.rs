#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, str::FromStr};

const GRID_WIDTH: usize = 1200;
const GRID_HEIGHT: usize = 200;

pub fn solve() -> SolutionPair {
    (Solution::UInt(part_1()), Solution::UInt(part_2()))
}

fn part_1() -> usize {
    let mut grid = create_grid();
    let mut sand_count = 0;
    loop {
        let mut sand = Coordinate { x: 500, y: 0 };
        drop_sand(&mut grid, &mut sand);
        sand_count += 1;
        if sand.y == GRID_HEIGHT - 1 {
            return sand_count - 1;
        }
    }
}

fn part_2() -> usize {
    let mut grid = create_grid();
    let mut lowest_floor = 0;
    for i in 0..grid.len() {
        if grid[i].iter().any(|&v| v) {
            lowest_floor = i;
        }
    }

    for i in 0..GRID_WIDTH {
        grid[lowest_floor + 2][i] = true;
    }

    let mut sand_count = 0;
    loop {
        let mut sand = Coordinate { x: 500, y: 0 };
        drop_sand(&mut grid, &mut sand);
        sand_count += 1;
        if sand.y == 0 {
            return sand_count;
        }
    }
}

fn drop_sand(grid: &mut [[bool; GRID_WIDTH]; GRID_HEIGHT], sand: &mut Coordinate) {
    let mut stuck = false;
    while let Some(new_location) = try_fall(&grid, &sand) {
        *sand = new_location;
    }

    if sand.y != GRID_HEIGHT - 1 {
        grid[sand.y][sand.x] = true;
    }
}

fn try_fall(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT], sand: &Coordinate) -> Option<Coordinate> {
    if sand.y == GRID_HEIGHT - 1 {
        return None;
    }
    if !grid[sand.y + 1][sand.x] {
        // straight down
        return Some(Coordinate {
            x: sand.x,
            y: sand.y + 1,
        });
    } else if !grid[sand.y + 1][sand.x - 1] {
        // down left
        return Some(Coordinate {
            x: sand.x - 1,
            y: sand.y + 1,
        });
    } else if !grid[sand.y + 1][sand.x + 1] {
        // down left
        return Some(Coordinate {
            x: sand.x + 1,
            y: sand.y + 1,
        });
    }
    None
}

fn create_grid() -> [[bool; GRID_WIDTH]; GRID_HEIGHT] {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
    read_to_string("input/day14")
        .expect("Could not read file")
        .lines()
        .for_each(|line| {
            let binding = line
                .split(" -> ")
                .map(|coordinate_str| coordinate_str.parse::<Coordinate>().unwrap())
                .collect::<Vec<_>>();
            let ranges = binding.windows(2);
            for range in ranges {
                if range[0].x == range[1].x {
                    let mut ys = [range[0].y, range[1].y];
                    ys.sort();
                    for y in ys[0]..=ys[1] {
                        grid[y][range[0].x] = true;
                    }
                } else {
                    // same y
                    let mut xs = [range[0].x, range[1].x];
                    xs.sort();
                    for x in xs[0]..=xs[1] {
                        grid[range[0].y][x] = true;
                    }
                }
            }
        });
    grid
}
#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        Ok(Self { x, y })
    }
}
