use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input).unwrap();
    part2(&input).unwrap();
}

fn part1(input: &str) -> Result<(), Box<Error>> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<Error>> {
    let mut freq = 0;
    let mut map: HashMap<i32, bool> = HashMap::new();
    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if map.contains_key(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            } else {
                map.insert(freq, true);
            }
        }
    }
}
