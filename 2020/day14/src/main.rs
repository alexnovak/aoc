use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn get_val(mask: &str, val: u64) -> u64 {
    let mut ret = val;
    let chars: Vec<char> = mask.chars().collect();
    for i in 0..36 {
        match chars[35 - i] {
            '1' => ret |= 1 << i,
            '0' => ret &= !(1 << i),
            _ => {}
        }
    }
    ret
}

fn build_data(input: &str) -> HashMap<u64, u64> {
    let mut data: HashMap<u64, u64> = HashMap::new();
    let mask_re = Regex::new(r"mask = (?P<mask>[01X]+)").unwrap();
    let write_re = Regex::new(r"mem\[(?P<key>[0-9]+)\] = (?P<val>[0-9]+)").unwrap();
    let mut mask: String = String::new();
    for line in input.lines() {
        if let Some(mask_cap) = mask_re.captures(line) {
            // set mask somehow
            mask = mask_cap["mask"].into();
        } else if let Some(write_cap) = write_re.captures(line) {
            let key = write_cap["key"].parse::<u64>().unwrap();
            let val = write_cap["val"].parse::<u64>().unwrap();
            let val = get_val(&mask, val);
            data.insert(key, val);
        }
    }
    data
}

fn part1(input: &str) {
    let data = build_data(input);

    let mut sum = 0;
    for v in data.values() {
        sum += v;
    }

    println!("{}", sum);
}

fn get_keys(mask: &str, key: u64) -> Vec<u64> {
    let mut key = key;
    let mut ret = Vec::new();
    let mut floats = Vec::new();
    let chars: Vec<char> = mask.chars().collect();
    for i in 0..36 {
        match chars[35 - i] {
            '1' => key |= 1 << i,
            'X' => {
                floats.push(i);
                key &= !(1 << i)
            }
            _ => {}
        }
    }
    for val in 0..2_usize.pow(floats.len() as u32) {
        let mut key_copy = key;
        for index in 0..floats.len() {
            key_copy |= (((val >> index) & 1) << floats[index]) as u64;
        }
        ret.push(key_copy);
    }
    ret
}

fn build_data2(input: &str) -> HashMap<u64, u64> {
    let mut data: HashMap<u64, u64> = HashMap::new();
    let mask_re = Regex::new(r"mask = (?P<mask>[01X]+)").unwrap();
    let write_re = Regex::new(r"mem\[(?P<key>[0-9]+)\] = (?P<val>[0-9]+)").unwrap();
    let mut mask: String = String::new();
    for line in input.lines() {
        if let Some(mask_cap) = mask_re.captures(line) {
            // set mask somehow
            mask = mask_cap["mask"].into();
        } else if let Some(write_cap) = write_re.captures(line) {
            let key = write_cap["key"].parse::<u64>().unwrap();
            let keys = get_keys(&mask, key);
            let val = write_cap["val"].parse::<u64>().unwrap();
            for k in keys {
                data.insert(k, val);
            }
        }
    }
    data
}

fn part2(input: &str) {
    let data = build_data2(input);

    let mut sum = 0;
    for v in data.values() {
        sum += v;
    }

    println!("{}", sum);
}
