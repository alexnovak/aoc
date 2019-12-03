use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn do_pair(input: &str, p1: u32, p2: u32) -> u32 {
    let mut instructions: Vec<u32> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    instructions[1] = p1;
    instructions[2] = p2;

    let mut index = 0;
    loop {
        let instruction = instructions[index];
        if instruction == 99 {
            break;
        }
        let left_location = instructions[index + 1] as usize;
        let right_location = instructions[index + 2] as usize;
        let left_op = instructions[left_location];
        let right_op = instructions[right_location];
        let write_location = instructions[index + 3] as usize;
        if instruction == 1 {
            instructions[write_location] = left_op + right_op;
        }
        if instruction == 2 {
            instructions[write_location] = left_op * right_op;
        }
        index += 4;
    }
    return instructions[0];
}

fn part1(input: &str) {
    let answer = do_pair(input, 12, 2);
    println!("{}", answer);
}

fn part2(input: &str) {
    let mut done = false;
    for p1 in 0..100 {
        for p2 in 0..100 {
            let result = do_pair(input, p1, p2);
            if result == 19690720 {
                println!("{}", 100 * p1 + p2);
                done = true;
            }
            if done {
                break;
            }
        }
        if done {
            break;
        }
    }
}
