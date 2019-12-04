use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn get_range(range: &str) -> (i64, i64) {
    let values_raw: Vec<&str> = range.trim().split("-").collect();
    let lower: i64 = values_raw[0].parse().unwrap();
    let upper: i64 = values_raw[1].parse().unwrap();
    (lower, upper)
}

fn meets_criteria(value: i64) -> bool {
    let mut has_double = false;
    let mut test_value = value;
    while test_value >= 10 {
        let right_cmp = test_value % 10;
        let left_cmp = (test_value / 10) % 10;
        if right_cmp == left_cmp {
            has_double = true;
        }
        if left_cmp > right_cmp {
            return false;
        }
        test_value = test_value / 10;
    }
    return has_double;
}

fn part1(input: &str) {
    let (lower, upper) = get_range(input);
    let mut sum: u64 = 0;
    for value in lower..upper + 1 {
        if meets_criteria(value) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn meets_criteria2(value: i64) -> bool {
    let mut has_double = false;
    let mut test_value = value;
    let mut last_digit = -1;
    while test_value >= 10 {
        let right_cmp = test_value % 10;
        let left_cmp = (test_value / 10) % 10;
        if left_cmp > right_cmp {
            return false;
        }
        if test_value >= 100 {
            if (test_value % 1000) % 111 == 0 {
                last_digit = right_cmp;
                test_value = test_value / 10;
                continue;
            }
        }
        if last_digit != -1 {
            if right_cmp == last_digit {
                last_digit = right_cmp;
                test_value = test_value / 10;
                continue;
            }
        }
        if right_cmp == left_cmp {
            has_double = true;
        }
        last_digit = right_cmp;
        test_value = test_value / 10;
    }
    return has_double;
}

fn part2(input: &str) {
    let (lower, upper) = get_range(input);
    let mut sum: u64 = 0;
    for value in lower..upper + 1 {
        if meets_criteria2(value) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
