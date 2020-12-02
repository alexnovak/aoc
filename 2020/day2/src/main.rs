use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut valid_passwords = 0;
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let minmaxtokens: Vec<&str> = tokens[0].split("-").collect();
        let min = minmaxtokens[0].parse::<i32>().unwrap();
        let max = minmaxtokens[1].parse::<i32>().unwrap();

        let letter = tokens[1].chars().next().unwrap();

        let mut count = 0;
        for c in tokens[2].chars() {
            if c == letter {
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
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let minmaxtokens: Vec<&str> = tokens[0].split("-").collect();
        let min = minmaxtokens[0].parse::<i32>().unwrap() as usize;
        let max = minmaxtokens[1].parse::<i32>().unwrap() as usize;

        let letter = tokens[1].chars().next().unwrap();

        if tokens[2].len() + 1 < max {
            continue;
        }

        let chars: Vec<char> = tokens[2].chars().collect();

        if (chars[min - 1] == letter) ^ (chars[max - 1] == letter) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
