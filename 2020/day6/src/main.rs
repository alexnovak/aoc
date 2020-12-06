use regex::{Captures, Regex};
use std::collections::HashSet;
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
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut simple = String::from(group);
        simple.retain(|c| !c.is_whitespace());
        let mut simple: Vec<char> = simple.chars().collect();
        simple.sort();
        simple.dedup();
        sum += simple.len();
    }
    println!("{}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut iter = group.lines();
        let first_question = iter.next();
        let mut s = HashSet::new();
        if let Some(first_string) = first_question {
            for c in first_string.chars() {
                s.insert(c);
            }
        }
        for question in iter {
            let mut s_prime = HashSet::new();
            for c in question.chars() {
                s_prime.insert(c);
            }
            s = s.intersection(&s_prime).copied().collect();
        }
        sum += s.len();
    }
    println!("{}", sum);
}
