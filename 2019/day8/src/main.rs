use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
}

fn part1(input: &str) {
    let input: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|o| o.is_some())
        .map(|s| s.unwrap())
        .collect();
    let width = 25;
    let height = 6;

    let mut counts: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for row in 0..height {
        for col in 0..width {
            let dict = counts.entry(row).or_insert(HashMap::new());
            let counter = dict.entry(input[(width * row + col) as usize]).or_insert(0);
            *counter += 1;
        }
    }
    let mut least_zeros_key: u32 = 0;
    for row in counts.keys() {
        let cur_least_zeros = match counts.get(&least_zeros_key) {
            Some(d) => match d.get(&0) {
                Some(u) => u,
                None => &0,
            },
            None => &0,
        };
        let dict = counts.get(row).unwrap();
        let row_zeros = match dict.get(row) {
            Some(u) => u,
            None => &0,
        };
        if *row_zeros < *cur_least_zeros {
            least_zeros_key = *row;
        }
    }

    println!("{}", least_zeros_key);
    for (row, dict) in counts.iter() {
        println!("Row {}: {:?}", row, dict);
    }
}
