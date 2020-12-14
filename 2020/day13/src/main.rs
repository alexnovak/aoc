use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn build_data(input: &str) -> (u64, Vec<u64>) {
    let mut iter = input.lines();
    let time = iter.next().unwrap().parse::<u64>().unwrap();
    let ids = iter
        .next()
        .unwrap()
        .split(',')
        .filter(|s| *s != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    (time, ids)
}

fn part1(input: &str) {
    let (time, ids) = build_data(input);

    let mut smallest_id = 0;
    let mut smallest = u64::MAX;
    for id in ids {
        let comp_val = id * (time / id + 1) - time;
        println!("{}, {}", id, comp_val);
        if comp_val < smallest {
            smallest_id = id;
            smallest = comp_val;
        }
    }

    println!("{}", smallest * smallest_id);
}

fn build_data2(input: &str) -> Vec<(i64, i64)> {
    let mut iter = input.lines();
    iter.next().unwrap().parse::<u64>().unwrap();
    let ids: Vec<&str> = iter.next().unwrap().split(',').collect();
    let mut right = 0;
    let mut ret: Vec<(i64, i64)> = Vec::new();
    for id in ids.iter() {
        if *id != "x" {
            let left = id.parse::<i64>().unwrap();
            ret.push((left, right));
            right += 1;
        } else {
            right += 1;
        }
    }
    ret
}

fn part2(input: &str) {
    let data = build_data2(input);

    let mut iter_val = 0;
    let mut prod = 1;
    println!("{:?}", data);
    for pair in data {
        println!("working on {}", pair.0);
        while (iter_val + pair.1) % pair.0 != 0 {
            iter_val += prod;
        }
        prod *= pair.0;
    }

    println!("{}", iter_val);
}
