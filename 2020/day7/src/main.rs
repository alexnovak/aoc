use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type BagMap = HashMap<String, Vec<(u32, String)>>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn contains_gold(map: &BagMap, bag: String) -> bool {
    let values = &map[&bag];
    let mut ret = false;
    for v in values {
        if v.1 == "shiny gold" {
            ret = true;
            break;
        } else {
            ret = contains_gold(&map, v.1.clone());
            if ret {
                break;
            }
        }
    }
    return ret;
}

fn build_map(input: &str) -> BagMap {
    let mut map = BagMap::new();
    for line in input.lines() {
        let color_regex =
            Regex::new(r"(?P<color>[a-z\s]+) bags contain (?P<subbags>[a-z,\s0-9]+).").unwrap();
        let color_caps = color_regex.captures(line).unwrap();
        let color = color_caps["color"].to_string();
        let subbags = color_caps["subbags"].to_string();
        let mut values: Vec<(u32, String)> = Vec::new();
        for subbag in subbags.split(",") {
            let re = Regex::new(r"(?P<count>\d+) (?P<color>[a-z\s]+) bags?").unwrap();
            let caps = re.captures(subbag);
            if let Some(subbag_info) = caps {
                let count = subbag_info["count"].parse::<u32>().unwrap();
                values.push((count, subbag_info["color"].to_string()));
            }
        }
        map.insert(color, values);
    }
    map
}

fn part1(input: &str) {
    // Build map
    let map = build_map(input);
    // Find values that eventually lead to gold
    let mut sum = 0;
    for key in map.keys() {
        if contains_gold(&map, key.clone()) {
            sum += 1
        }
    }

    println!("{}", sum);
}

fn count_subbags(map: &BagMap, bag: String) -> u32 {
    let values = &map[&bag];
    if values.is_empty() {
        return 0;
    }
    let mut sum = 0;
    for v in values {
        sum += v.0 * (1 + count_subbags(map, v.1.clone()));
    }
    sum
}

fn part2(input: &str) {
    let map = build_map(input);
    let ans = count_subbags(&map, "shiny gold".into());
    println!("{}", ans);
}
