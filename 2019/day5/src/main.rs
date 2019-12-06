use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

#[derive(PartialEq, Debug)]
enum ParamMode {
    Position,
    Immediate,
}

fn parse_instruction(instruction: i32) -> (i32, [ParamMode; 3]) {
    let ret_instruction = instruction % 100;
    let mut p1mode = ParamMode::Position;
    let mut p2mode = ParamMode::Position;
    let mut p3mode = ParamMode::Position;
    if (instruction / 100) % 10 == 1 {
        p1mode = ParamMode::Immediate;
    }
    if (instruction / 1000) % 10 == 1 {
        p2mode = ParamMode::Immediate;
    }
    if (instruction / 10000) % 10 == 1 {
        p3mode = ParamMode::Immediate;
    }
    (ret_instruction, [p1mode, p2mode, p3mode])
}

fn part1(input: &str) {
    let answer = run_program(input, 1);
    println!("{}", answer);
}

fn run_program(input: &str, input_num: i32) -> i32 {
    let mut instructions: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut index = 0;
    let input = input_num;
    let mut output = -1;
    loop {
        let (instruction, modes) = parse_instruction(instructions[index]);
        //println!("instruction: {}, modes: {:?}", instruction, modes);
        if instruction == 99 {
            break;
        }
        let mut values = [0; 3];
        //println!("{:?}", &instructions[index..index + 3]);
        if instruction <= 2 {
            for value_iter in 0..2 {
                if modes[value_iter] == ParamMode::Position {
                    let position = instructions[index + value_iter + 1] as usize;
                    values[value_iter] = instructions[position];
                } else {
                    values[value_iter] = instructions[index + value_iter + 1];
                }
            }
            //println!("{:?}", &instructions[index..index + 4]);
            let write_location = instructions[index + 3] as usize;
            if instruction == 1 {
                instructions[write_location] = values[0] + values[1];
            }
            if instruction == 2 {
                instructions[write_location] = values[0] * values[1];
            }
            index += 4;
        } else if instruction <= 4 {
            //println!("{:?}", &instructions[index..index + 2]);
            let write_location = instructions[index + 1] as usize;
            if instruction == 3 {
                instructions[write_location] = input;
            }
            if instruction == 4 {
                output = instructions[write_location];
                println!("TEST OUTPUT: {}", output);
            }
            index += 2;
        } else if instruction <= 6 {
            for value_iter in 0..2 {
                if modes[value_iter] == ParamMode::Position {
                    let position = instructions[index + value_iter + 1] as usize;
                    values[value_iter] = instructions[position];
                } else {
                    values[value_iter] = instructions[index + value_iter + 1];
                }
            }
            if instruction == 5 {
                if values[0] != 0 {
                    index = values[1] as usize;
                } else {
                    index += 3;
                }
                continue;
            }
            if instruction == 6 {
                if values[0] == 0 {
                    index = values[1] as usize;
                } else {
                    index += 3;
                }
                continue;
            }
        } else {
            for value_iter in 0..2 {
                if modes[value_iter] == ParamMode::Position {
                    let position = instructions[index + value_iter + 1] as usize;
                    values[value_iter] = instructions[position];
                } else {
                    values[value_iter] = instructions[index + value_iter + 1];
                }
            }
            let write_location = instructions[index + 3] as usize;
            if instruction == 7 {
                if values[0] < values[1] {
                    instructions[write_location] = 1;
                } else {
                    instructions[write_location] = 0;
                }
            }
            if instruction == 8 {
                if values[0] == values[1] {
                    instructions[write_location] = 1;
                } else {
                    instructions[write_location] = 0;
                }
            }
            index += 4;
        }
    }
    return output;
}
fn part2(input: &str) {
    let answer = run_program(input, 5);
    println!("{}", answer);
}
