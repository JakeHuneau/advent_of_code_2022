#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let mut total = 0;
    let grid = read_to_string("input/day8")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut max_tree_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let this_count = count_trees(&grid, i, j);
            if this_count > max_tree_count {
                max_tree_count = this_count;
            }
            if is_visible(&grid, i, j) {
                total += 1;
            }
        }
    }
    (Solution::UInt(total), Solution::UInt(max_tree_count))
}

fn is_visible(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let this_val = grid[row][column];
    if row == 0 || row == grid.len() - 1 || column == 0 || column == grid[0].len() - 1 {
        return true;
    }
    // Need to check every tree in a direction
    let mut visible_left = true;
    for c in (0..column).rev() {
        if this_val <= grid[row][c] {
            visible_left = false;
            break;
        }
    }
    if visible_left {
        return true;
    }

    let mut visible_right = true;
    for c in column + 1..grid.len() {
        if this_val <= grid[row][c] {
            visible_right = false;
            break;
        }
    }
    if visible_right {
        return true;
    }

    let mut visible_up = true;
    for r in (0..row).rev() {
        if this_val <= grid[r][column] {
            visible_up = false;
            break;
        }
    }
    if visible_up {
        return true;
    }

    let mut visible_down = true;
    for r in row + 1..grid[0].len() {
        if this_val <= grid[r][column] {
            visible_down = false;
            break;
        }
    }
    if visible_down {
        return true;
    }
    false
}

fn count_trees(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> usize {
    let this_val = grid[row][column];

    // Need to check every tree in a direction
    let mut left_count = 0;
    for c in (0..column).rev() {
        left_count += 1;
        if this_val <= grid[row][c] {
            break;
        }
    }

    let mut right_count = 0;
    for c in column + 1..grid.len() {
        right_count += 1;
        if this_val <= grid[row][c] {
            break;
        }
    }

    let mut up_count = 0;
    for r in (0..row).rev() {
        up_count += 1;
        if this_val <= grid[r][column] {
            break;
        }
    }

    let mut down_count = 0;
    for r in row + 1..grid[0].len() {
        down_count += 1;
        if this_val <= grid[r][column] {
            break;
        }
    }
    left_count * right_count * up_count * down_count
}
