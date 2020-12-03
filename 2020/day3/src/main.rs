use regex::Regex;
use std::io::{self, Read};

#[macro_use]
extern crate scan_fmt;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

enum Cell {
    Open,
    Tree,
}

fn calculate_hits(grid: &Vec<Vec<char>>, x_diff: usize, y_diff: usize) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut trees = 0;
    let mut dist_x = x_diff;
    let mut dist_y = y_diff;

    while (dist_y < height) {
        if grid[dist_y][dist_x % width] == '#' {
            trees += 1;
        }
        dist_x += x_diff;
        dist_y += y_diff
    }

    return trees;
}

fn part1(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    println!("{}", calculate_hits(&grid, 3, 1));
}

fn part2(input: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let mut mul: i64 = 1;
    for pair in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        mul *= calculate_hits(&grid, pair.0, pair.1) as i64;
    }
    println!("{}", mul);
}
