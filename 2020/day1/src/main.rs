use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut answer = 0;
    for line in input.lines() {
        let val = line.parse::<i32>().unwrap();
        let key = 2020 - val;
        if seen.contains(&key) {
            answer = key * val;
            break;
        } else {
            seen.insert(val);
        }
    }
    println!("part1: {}", answer)
}

fn part2(input: &str) {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut answer = 0;
    for line in input.lines() {
        let val = line.parse::<i32>().unwrap();
        for old_val in seen.iter() {
            let key = 2020 - val - old_val;
            if seen.contains(&key) {
                answer = key * val * old_val;
                break;
            }
        }
        if answer != 0 {
            break;
        }
        seen.insert(val);
    }
    println!("part2: {}", answer)
}
