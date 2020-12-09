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
    let mut data: Vec<u64> = Vec::new();
    for line in input.lines() {
        data.push(line.parse::<u64>().unwrap());
    }
    let preamble = 25;
    'outer: for i in preamble..data.len() {
        let mut found = false;
        for j in i - preamble..i {
            for k in j + 1..i {
                if data[i] == data[j] + data[k] {
                    found = true;
                    continue 'outer;
                }
            }
        }
        if !found {
            println!("{}", data[i]);
        }
    }
}

fn part2(input: &str) {
    let mut data: Vec<u64> = Vec::new();
    for line in input.lines() {
        data.push(line.parse::<u64>().unwrap());
    }

    'outer: for i in 0..data.len() {
        let start = i;
        let mut sum = data[start];
        let mut end = start + 1;
        let mut max = data[start];
        let mut min = data[start];
        loop {
            sum += data[end];
            if data[end] < min {
                min = data[end];
            }
            if data[end] > max {
                max = data[end]
            }
            if sum == 1492208709 {
                println!("{}, {}", start, end);
                println!("{}", max + min);
                println!("{}", sum);
            }
            if sum > 1492208709 {
                break;
            }
            end += 1;
        }
    }
}
