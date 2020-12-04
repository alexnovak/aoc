use regex::{Captures, Regex};
use std::io::{self, Read};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug)]
struct Pass {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Pass {
    fn is_valid(&self) -> bool {
        if !(1920 <= self.byr && self.byr <= 2002) {
            println!("{} bad", self.byr);
            return false;
        }

        if !(2010 <= self.iyr && self.iyr <= 2020) {
            return false;
        }

        if !(2020 <= self.eyr && self.eyr <= 2030) {
            return false;
        }

        let hgt_re = Regex::new(r"(?P<height>\d+)(?P<unit>[a-z]{2})").unwrap();
        if let Some(height_caps) = hgt_re.captures(&self.hgt) {
            let height = (&height_caps["height"]).parse::<u32>().unwrap();
            if &height_caps["unit"] == "cm" {
                if !(150 <= height && height <= 193) {
                    return false;
                }
            } else if &height_caps["unit"] == "in" {
                if !(59 <= height && height <= 76) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        let hcl_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
        if !hcl_re.is_match(&self.hcl) {
            return false;
        }

        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&self.ecl[..]) {
            return false;
        }

        if self.pid.len() != 9 {
            return false;
        }

        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn has_fields(s: &str) -> bool {
    for pattern in ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"].iter() {
        if !s.contains(pattern) {
            return false;
        }
    }
    true
}

fn part1(input: &str) {
    let re = Regex::new(r"[a-zA-Z0-9]\n([a-zA-Z])").unwrap();
    let result = re.replace_all(input, |caps: &Captures| format!(" {}", &caps[1]));
    let mut count = 0;
    for line in result.lines() {
        if has_fields(line) {
            count += 1
        }
    }
    println!("{}", count);
}

fn get_res_for_pattern(s: &String, p: &str) -> Option<String> {
    let re = Regex::new(&format!(r".*\s?{}:(?P<value>[^\s]+)[\s\n]?", p)).unwrap();
    let caps = re.captures(&s)?;
    return Some((&caps["value"]).into());
}

fn build_passport(s: String) -> Option<Pass> {
    // We assume that the string is valid (bad I know but eh)
    let byr = get_res_for_pattern(&s, "byr")?;
    let byr = byr.parse::<u32>().unwrap();
    let iyr = get_res_for_pattern(&s, "iyr")?;
    let iyr = iyr.parse::<u32>().unwrap();
    let eyr = get_res_for_pattern(&s, "eyr")?;
    let eyr = eyr.parse::<u32>().unwrap();
    let hgt = get_res_for_pattern(&s, "hgt")?;
    let hcl = get_res_for_pattern(&s, "hcl")?;
    let ecl = get_res_for_pattern(&s, "ecl")?;
    let pid = get_res_for_pattern(&s, "pid")?;
    let mut cid = None;
    if s.contains("cid:") {
        cid = Some(get_res_for_pattern(&s, "cid"))?;
    }

    Some(Pass {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid,
    })
}

fn part2(input: &str) {
    let re = Regex::new(r"([a-zA-Z0-9])\n([a-zA-Z])").unwrap();
    let result = re.replace_all(input, |caps: &Captures| {
        format!("{} {}", &caps[1], &caps[2])
    });
    let result = result.replace("\n\n", "\n");
    let mut valid_count = 0;
    for line in result.lines() {
        if has_fields(line) {
            if let Some(pass) = build_passport(line.into()) {
                if pass.is_valid() {
                    valid_count += 1;
                }
            }
        }
    }
    println!("{}", valid_count);
}
