use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut valid_passwords = 0;
    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<letter>[a-z]): (?P<pass>[a-z]*)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let min = caps["min"].parse::<i32>().unwrap();
        let max = caps["max"].parse::<i32>().unwrap();

        let mut count = 0;
        for c in caps["pass"].chars() {
            if c == caps["letter"].chars().next().unwrap() {
                count += 1;
            }
        }

        if min <= count && count <= max {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}

fn part2(input: &str) {
    let mut valid_passwords = 0;
    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<letter>[a-z]): (?P<pass>[a-z]*)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let min = caps["min"].parse::<i32>().unwrap() as usize;
        let max = caps["max"].parse::<i32>().unwrap() as usize;

        if caps["pass"].len() + 1 < max {
            continue;
        }

        let chars: Vec<char> = caps["pass"].chars().collect();
        let letter = caps["letter"].chars().next().unwrap();

        if (chars[min - 1] == letter) ^ (chars[max - 1] == letter) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
