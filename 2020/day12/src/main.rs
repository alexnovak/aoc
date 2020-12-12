use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut data: Vec<(char, i32)> = Vec::new();
    let re = Regex::new(r"(?P<dir>[A-Z])(?P<count>[0-9]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        data.push((
            caps["dir"].chars().collect::<Vec<char>>()[0],
            caps["count"].parse::<i32>().unwrap(),
        ));
    }
    let mut pos = (0, 0);
    let mut angle = 0;
    for inst in data {
        match inst.0 {
            'N' => pos.1 += inst.1,
            'S' => pos.1 -= inst.1,
            'E' => pos.0 += inst.1,
            'W' => pos.0 -= inst.1,
            'L' => angle = (angle + inst.1).rem_euclid(360),
            'R' => angle = (angle - inst.1).rem_euclid(360),
            'F' => match angle {
                0 => pos.0 += inst.1,
                90 => pos.1 += inst.1,
                180 => pos.0 -= inst.1,
                270 => pos.1 -= inst.1,
                _ => println!("bad angle {}", angle),
            },
            _ => println!("band inst {}", inst.0),
        }
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}

fn rotate_point(angle: i32, pos: (i32, i32)) -> (i32, i32) {
    match angle {
        90 => (pos.1, -pos.0),
        180 => (-pos.0, -pos.1),
        270 => (-pos.1, pos.0),
        _ => (0, 0),
    }
}

fn part2(input: &str) {
    let mut data: Vec<(char, i32)> = Vec::new();
    let re = Regex::new(r"(?P<dir>[A-Z])(?P<count>[0-9]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        data.push((
            caps["dir"].chars().collect::<Vec<char>>()[0],
            caps["count"].parse::<i32>().unwrap(),
        ));
    }
    let mut way_pos = (10, 1);
    let mut pos = (0, 0);
    for inst in data {
        match inst.0 {
            'N' => way_pos.1 += inst.1,
            'S' => way_pos.1 -= inst.1,
            'E' => way_pos.0 += inst.1,
            'W' => way_pos.0 -= inst.1,
            'L' => way_pos = rotate_point((360 - inst.1), way_pos),
            'R' => way_pos = rotate_point(inst.1, way_pos),
            'F' => {
                pos.0 += way_pos.0 * inst.1;
                pos.1 += way_pos.1 * inst.1;
            }
            _ => println!("band inst {}", inst.0),
        }
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}
