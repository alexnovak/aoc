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
    let mut jump1 = 0;
    let mut jump3 = 1;
    data.push(0);
    data.sort();
    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];
        if diff == 1 {
            jump1 += 1;
        } else if diff == 3 {
            jump3 += 1;
        }
    }
    println!("{}", jump1 * jump3);
}

fn count_paths(input: &Vec<u64>, cache: &mut HashMap<usize, u64>, index: usize) -> u64 {
    if cache.contains_key(&index) {
        return *(cache.get(&index).unwrap());
    }
    if index == input.len() - 1 {
        return 1;
    }
    let mut sum = 0;
    for i in 1..4 {
        if index + i >= input.len() {
            continue;
        }
        if input[index + i] - input[index] <= 3 {
            sum += count_paths(input, cache, index + i);
        }
    }
    cache.insert(index, sum);
    return sum;
}

fn part2(input: &str) {
    let mut data: Vec<u64> = Vec::new();
    let mut cache: HashMap<usize, u64> = HashMap::new();
    for line in input.lines() {
        data.push(line.parse::<u64>().unwrap());
    }
    data.push(0);
    data.sort();
    data.push(data.last().unwrap() + 3);
    println!("{}", count_paths(&data, &mut cache, 0));
}
