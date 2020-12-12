use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn get_adjacent(i: usize, j: usize, h: i64, w: i64) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            if i as i64 + x >= 0
                && i as i64 + x < h
                && j as i64 + y >= 0
                && j as i64 + y < w
                && !(x == 0 && y == 0)
            {
                coords.push(((i as i64 + x) as usize, (j as i64 + y) as usize));
            }
        }
    }
    coords
}

fn count_adjacent(state: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    let h = state.len();
    let w = state[0].len();
    let ret = get_adjacent(i, j, h as i64, w as i64)
        .iter()
        .map(|coord| {
            if state[coord.0][coord.1] == '#' {
                return 1_i64;
            } else {
                return 0_i64;
            }
        })
        .sum();
    ret
}

fn tick(state: &mut Vec<Vec<char>>) -> (i64, bool) {
    let state_old = state.clone();
    let mut occupied_seats = 0_i64;
    let mut changed = false;
    for i in 0..state_old.len() {
        for j in 0..state_old[i].len() {
            match state_old[i][j] {
                'L' => {
                    if count_adjacent(&state_old, i, j) == 0 {
                        state[i][j] = '#';
                        changed = true;
                        occupied_seats += 1;
                    }
                }
                '#' => {
                    if count_adjacent(&state_old, i, j) >= 4 {
                        state[i][j] = 'L';
                        changed = true;
                    } else {
                        occupied_seats += 1;
                    }
                }
                _ => {}
            }
        }
    }
    (occupied_seats, changed)
}

fn part1(input: &str) {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        data.push(line.chars().collect());
    }
    let mut changed = true;
    let mut occupied_seats = 0;
    while changed {
        let result = tick(&mut data);
        occupied_seats = result.0;
        changed = result.1;
    }
    let result = tick(&mut data);
    println!("{}", result.0);
}

fn count_view(state: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    let h = state.len() as i64;
    let w = state[0].len() as i64;
    let mut count = 0;
    for step in vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ] {
        let mut pos = (i as i64, j as i64);
        loop {
            pos.0 += step.0;
            pos.1 += step.1;
            if !(0 <= pos.0 && pos.0 < h && 0 <= pos.1 && pos.1 < w) {
                break;
            }
            if state[pos.0 as usize][pos.1 as usize] == 'L' {
                break;
            }
            if state[pos.0 as usize][pos.1 as usize] == '#' {
                count += 1;
                break;
            }
        }
    }
    count
}

fn tick2(state: &mut Vec<Vec<char>>) -> (i64, bool) {
    let state_old = state.clone();
    let mut occupied_seats = 0_i64;
    let mut changed = false;
    for i in 0..state_old.len() {
        for j in 0..state_old[i].len() {
            match state_old[i][j] {
                'L' => {
                    if count_view(&state_old, i, j) == 0 {
                        state[i][j] = '#';
                        changed = true;
                        occupied_seats += 1;
                    }
                }
                '#' => {
                    if count_view(&state_old, i, j) >= 5 {
                        state[i][j] = 'L';
                        changed = true;
                    } else {
                        occupied_seats += 1;
                    }
                }
                _ => {}
            }
        }
    }
    (occupied_seats, changed)
}

fn part2(input: &str) {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        data.push(line.chars().collect());
    }
    let mut changed = true;
    while changed {
        let result = tick2(&mut data);
        changed = result.1;
    }
    let result = tick2(&mut data);
    println!("{}", result.0);
}
