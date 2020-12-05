use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn get_id(s: &str) -> u32 {
    return s.chars().take(10).fold(0, |acc, x| {
        if x == 'B' || x == 'R' {
            acc * 2 + 1
        } else {
            acc * 2
        }
    });
}

fn part1(input: &str) {
    let mut max = 2;
    for line in input.lines() {
        let id = get_id(line);
        if id > max {
            max = id
        }
    }
    println!("{}", max);
}

fn part2(input: &str) {
    let mut seats = Vec::new();
    for line in input.lines() {
        let id = get_id(line);
        seats.push(id);
    }
    seats.sort();
    for i in 0..seats.len() - 1 {
        if seats[i + 1] - seats[i] > 1 {
            println!("{}", seats[i] + 1);
        }
    }
}
