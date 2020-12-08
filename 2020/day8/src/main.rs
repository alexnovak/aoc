use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut inst: Vec<(String, i32)> = Vec::new();
    for line in input.lines() {
        let info: Vec<&str> = line.split_whitespace().collect();
        let code = info[0];
        let num = info[1].parse::<i32>().unwrap();
        inst.push((code.into(), num));
    }
    let mut acc = 0_i32;
    let mut sp = 0_i32;
    let mut seen: HashSet<i32> = HashSet::new();
    loop {
        if seen.contains(&sp) {
            break;
        }
        seen.insert(sp);
        let cur = &inst[sp as usize];
        match &cur.0[..] {
            "nop" => sp += 1,
            "acc" => {
                sp += 1;
                acc += cur.1;
            }
            "jmp" => {
                sp = sp + cur.1;
            }
            _ => {
                println!("uh oh undefined behavior");
                break;
            }
        }
    }
    println!("{}", acc);
}

fn part2(input: &str) {
    let mut inst: Vec<(String, i32)> = Vec::new();
    for line in input.lines() {
        let info: Vec<&str> = line.split_whitespace().collect();
        let code = info[0];
        let num = info[1].parse::<i32>().unwrap();
        inst.push((code.into(), num));
    }
    let prog_size = inst.len();

    for trial in inst
        .clone()
        .into_iter()
        .enumerate()
        .filter(|i| &i.1 .0[..] == "nop" || &i.1 .0[..] == "jmp")
    {
        let mut mod_inst = inst.clone();
        if &trial.1 .0[..] == "nop" {
            let old = &mod_inst[trial.0];
            mod_inst[trial.0] = ("jmp".into(), old.1);
        } else {
            let old = &mod_inst[trial.0];
            mod_inst[trial.0] = ("nop".into(), old.1);
        }

        let mut acc = 0_i32;
        let mut sp = 0_i32;
        let mut seen: HashSet<i32> = HashSet::new();
        loop {
            if sp as usize == prog_size {
                println!("Succeeded! Acc is {}", acc);
                break;
            }
            let cur = &mod_inst[sp as usize];
            if seen.contains(&sp) && &cur.0[..] == "jmp" {
                break;
            }
            seen.insert(sp);
            match &cur.0[..] {
                "nop" => sp += 1,
                "acc" => {
                    sp += 1;
                    acc += cur.1;
                }
                "jmp" => {
                    sp = sp + cur.1;
                }
                _ => {
                    println!("uh oh undefined behavior");
                    break;
                }
            }
        }
    }
}
