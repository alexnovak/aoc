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
    let mut two_repeat_words = 0;
    let mut three_repeat_words = 0;
    for line in input.lines() {
        let mut counter: u64 = 0;
        let mut had_two = false;
        let mut had_three = false;
        for c in line.chars() {
            // Use hashmaps? Bah, way too readable!
            // Realize that we only need to count up to 4,
            // so we can encode the count of each of the 26 possible characters
            // in an integer by counting in base5.
            //
            // To anyone reading this, I would never, ever, never forever
            // do this in production code.
            let val: u32 = c as u32;
            let exponent = val - 97;
            if (counter / 5u64.pow(exponent)) % 5 != 4 {
                counter += 5u64.pow(exponent);
            }
        }
        for divisor in 0..26 {
            let remainder = (counter / 5u64.pow(divisor)) % 5;
            match remainder {
                2 => {
                    if !had_two {
                        two_repeat_words += 1;
                        had_two = true;
                    }
                }
                3 => {
                    if !had_three {
                        three_repeat_words += 1;
                        had_three = true;
                    }
                }
                _ => (),
            }
        }
    }
    writeln!(io::stdout(), "{}", two_repeat_words * three_repeat_words)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<Error>> {
    let mut map: HashMap<String, bool> = HashMap::new();
    for line in input.lines() {
        for index in 0..line.len() {
            let mut mod_line = line.to_string();
            mod_line.remove(index);
            if map.contains_key(&mod_line) {
                writeln!(io::stdout(), "{}", mod_line)?;
                return Ok(());
            } else {
                map.insert(mod_line, true);
            }
        }
    }
    Ok(())
}
