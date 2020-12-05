use regex::{Captures, Regex};
use std::io::{self, Read};

#[macro_use]
extern crate scan_fmt;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut max = 2;
    for line in input.lines() {
        let row = line
            .chars()
            .take(7)
            .fold(0, |acc, x| if x == 'B' { acc * 2 + 1 } else { acc * 2 });
        let col =
            line.chars()
                .skip(7)
                .take(3)
                .fold(0, |acc, x| if x == 'R' { acc * 2 + 1 } else { acc * 2 });
        let id = row * 8 + col;
        if id > max {
            max = id
        }
    }
    println!("{}", max);
}

fn part2(input: &str) {
    let mut seats = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .take(7)
            .fold(0, |acc, x| if x == 'B' { acc * 2 + 1 } else { acc * 2 });
        let col =
            line.chars()
                .skip(7)
                .take(3)
                .fold(0, |acc, x| if x == 'R' { acc * 2 + 1 } else { acc * 2 });
        let id = row * 8 + col;
        seats.push(id);
    }
    seats.sort();
    for i in 0..seats.len() - 1 {
        if seats[i + 1] - seats[i] > 1 {
            println!("{}", seats[i] + 1);
        }
    }
}
