use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x_location: i32,
    y_location: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
}

fn parse_line(line: &str) -> (u32, u32, u32, u32, u32) {
    let _line = line.to_string();
    let tokens: Vec<&str> = _line.split(' ').collect();
    let number: u32 = tokens[0].split_at(0).1.parse().unwrap();
    let y_loc: u32 = tokens[2].split(',');
    return (number, 1, 1, 1, 1);
}

fn part1(input: &str) -> Result<(), Box<Error>> {
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    for line in input.lines() {
        parse_line(line);
    }
    Ok(())
}
