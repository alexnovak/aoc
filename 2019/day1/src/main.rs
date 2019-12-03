use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut fuel = 0;
    for line in input.lines() {
        let weight: u64 = line.parse().unwrap();
        fuel += (weight / 3) - 2;
    }
    println!("{}", fuel);
}

fn part2(input: &str) {
    let mut fuel = 0;
    for line in input.lines() {
        let mut weight: i64 = line.parse().unwrap();
        while weight > 0 {
            let new_fuel = (weight / 3) - 2;
            if new_fuel > 0 {
                fuel += new_fuel;
            }
            weight = new_fuel;
        }
    }
    println!("{}", fuel);
}
