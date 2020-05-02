use std::collections::VecDeque;
use std::env;
use std::io::{self, Read};
use std::process;

static mut DEBUG: bool = false;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("ERROR, specify which part.");
        process::exit(1);
    }

    if args.len() > 2 && args[2] == "debug" {
        unsafe {
            DEBUG = true;
        }
    }

    if args[1].trim() != "2" {
        println! {"Running for problem 1:"}
        part1(&input);
    } else {
        println! {"Running for problem 2:"}
        part2(&input);
    }
}

#[derive(PartialEq, Debug, Clone)]
enum InstructionType {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    Invalid,
}

#[derive(Debug)]
struct Instruction {
    itype: InstructionType,
    params: Vec<i32>,
}

impl Instruction {
    fn add(&self, program: &mut Program) {
        let write_position = self.params[2] as usize;
        program.program[write_position] = self.params[0] + self.params[1];
        program.index += 4;
    }
    fn mult(&self, program: &mut Program) {
        let write_position = self.params[2] as usize;
        program.program[write_position] = self.params[0] * self.params[1];
        program.index += 4;
    }
    fn input(&self, program: &mut Program) {
        let write_position = self.params[0] as usize;
        program.program[write_position] = program.inputs.pop_front().unwrap();
        program.index += 2;
    }
    fn output(&self, program: &mut Program) {
        let read_location = self.params[0] as usize;
        program.output = program.program[read_location];
        program.state = ProgramState::Output;
        program.index += 2;
    }
    fn jump_if_true(&self, program: &mut Program) {
        if self.params[0] != 0 {
            program.index = self.params[1] as usize;
        } else {
            program.index += 3;
        }
    }
    fn jump_if_false(&self, program: &mut Program) {
        if self.params[0] == 0 {
            program.index = self.params[1] as usize;
        } else {
            program.index += 3;
        }
    }
    fn less_than(&self, program: &mut Program) {
        let write_location = self.params[2] as usize;
        if self.params[0] < self.params[1] {
            program.program[write_location] = 1;
        } else {
            program.program[write_location] = 0;
        }
        program.index += 4;
    }
    fn equals(&self, program: &mut Program) {
        let write_location = self.params[2] as usize;
        if self.params[0] == self.params[1] {
            program.program[write_location] = 1;
        } else {
            program.program[write_location] = 0;
        }
        program.index += 4;
    }
    fn invalid(&self, program: &mut Program) {
        eprintln!("ERORR: Received invalid instruction.");
        eprintln!(
            "Found int code {} at index {}",
            program.program[program.index], program.index
        );
        process::exit(1);
    }

    fn run(&self, mut program: &mut Program) {
        match self.itype {
            InstructionType::Add => self.add(&mut program),
            InstructionType::Multiply => self.mult(&mut program),
            InstructionType::Input => self.input(&mut program),
            InstructionType::Output => self.output(&mut program),
            InstructionType::JumpIfTrue => self.jump_if_true(&mut program),
            InstructionType::JumpIfFalse => self.jump_if_false(&mut program),
            InstructionType::LessThan => self.less_than(&mut program),
            InstructionType::Equals => self.equals(&mut program),
            InstructionType::Halt => {}
            InstructionType::Invalid => self.invalid(&mut program),
        }
    }
}

fn get_params(program: &Vec<i32>, index: usize, itype: InstructionType) -> Vec<i32> {
    let param_size = match itype {
        InstructionType::Add => 2,
        InstructionType::Multiply => 2,
        InstructionType::Input => 0,
        InstructionType::Output => 0,
        InstructionType::JumpIfTrue => 2,
        InstructionType::JumpIfFalse => 2,
        InstructionType::LessThan => 2,
        InstructionType::Equals => 2,
        InstructionType::Halt => 0,
        InstructionType::Invalid => 0,
    };
    let has_position = match itype {
        InstructionType::Add => true,
        InstructionType::Multiply => true,
        InstructionType::Input => true,
        InstructionType::Output => true,
        InstructionType::JumpIfTrue => false,
        InstructionType::JumpIfFalse => false,
        InstructionType::LessThan => true,
        InstructionType::Equals => true,
        InstructionType::Halt => false,
        InstructionType::Invalid => false,
    };

    let instruction = program[index];
    let mut params = Vec::new();
    let mut divisor = 100;
    for ipointer in index + 1..index + 1 + param_size {
        if (instruction / divisor) % 10 == 1 {
            let value = program[ipointer];
            params.push(value);
        } else {
            let position = program[ipointer] as usize;
            let value = program[position];
            params.push(value);
        }
        divisor *= 10;
    }
    if has_position {
        params.push(program[index + 1 + param_size])
    }
    params
}

fn parse_instruction(program: &Vec<i32>, index: usize) -> Instruction {
    let instruction = program[index];
    let itype = match instruction % 100 {
        1 => InstructionType::Add,
        2 => InstructionType::Multiply,
        3 => InstructionType::Input,
        4 => InstructionType::Output,
        5 => InstructionType::JumpIfTrue,
        6 => InstructionType::JumpIfFalse,
        7 => InstructionType::LessThan,
        8 => InstructionType::Equals,
        99 => InstructionType::Halt,
        _ => InstructionType::Invalid,
    };
    let params = get_params(program, index, itype.clone());
    return Instruction { itype, params };
}

#[derive(PartialEq)]
enum ProgramState {
    Unstarted,
    Output,
    Halted,
}

struct Program {
    state: ProgramState,
    program: Vec<i32>,
    inputs: VecDeque<i32>,
    index: usize,
    output: i32,
}

impl Program {
    fn new(program: &str) -> Program {
        let state = ProgramState::Unstarted;
        let program: Vec<i32> = program
            .trim()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let inputs: VecDeque<i32> = VecDeque::new();
        let index = 0;
        let output = -1;
        return Program {
            state,
            program,
            inputs,
            index,
            output,
        };
    }

    fn add_input(&mut self, input: i32) {
        self.inputs.push_back(input);
    }

    fn run_program(&mut self) -> (i32, ProgramState) {
        loop {
            let instruction = parse_instruction(&self.program, self.index);
            if unsafe { DEBUG } {
                println!("Running instruction: {:?}", instruction);
                println!("Program state:");
                println!("{:?}", self.program);
            }
            instruction.run(self);
            if instruction.itype == InstructionType::Halt {
                return (0, ProgramState::Halted);
            } else if instruction.itype == InstructionType::Output {
                return (self.output, ProgramState::Output);
            }
        }
    }
}

fn gen_permutation(base: [i32; 5]) -> Vec<[i32; 5]> {
    let mut result = Vec::new();
    let mut base = base.clone();
    let mut stack = [0; 5];

    result.push(base.clone());

    let mut index = 0;
    while index < 5 {
        if stack[index] < index {
            if index % 2 == 0 {
                base.swap(0, index);
            } else {
                base.swap(stack[index], index);
            }
            result.push(base.clone());
            stack[index] += 1;
            index = 0;
        } else {
            stack[index] = 0;
            index += 1;
        }
    }

    result
}

fn part1(input: &str) {
    let mut max = 0;
    for permutation in gen_permutation([0, 1, 2, 3, 4]) {
        let mut last_output = 0;
        for phase in &permutation {
            let args = [*phase, last_output].to_vec();
            let mut prog = Program::new(input);
            for arg in args.iter() {
                prog.add_input(*arg);
            }
            last_output = prog.run_program().0;
        }
        if last_output > max {
            max = last_output;
        }
    }
    println!("{}", max);
}

fn part2(input: &str) {
    let mut max = 0;
    for permutation in gen_permutation([5, 6, 7, 8, 9]) {
        let mut amplifiers: Vec<Program> = Vec::new();

        // Generate programs for each amplifier.
        for i in 0..5 {
            amplifiers.push(Program::new(input.clone()));
            amplifiers[i].add_input(permutation[i]);
        }

        // Seed first amplifier with 0.
        amplifiers[0].add_input(0);

        // Amplifiers go brrrrrrr
        let mut index = 0;
        loop {
            let output = amplifiers[index].run_program();
            if output.1 == ProgramState::Halted {
                let last_output = amplifiers[4].output;
                if last_output > max {
                    max = last_output;
                }
                break;
            }
            let next_index = (index + 1) % 5;
            amplifiers[next_index].add_input(output.0);
            index = next_index;
        }
    }
    println!("{}", max);
}
