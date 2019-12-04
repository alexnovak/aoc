use std::io::{self, Read};
use std::iter::Iterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn is_zero(&self) -> bool {
        return self.x == 0 && self.y == 0;
    }
}

fn d(p1: Pair, p2: Pair) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn id(p: Pair) -> i32 {
    d(Pair { x: 0, y: 0 }, p)
}

fn parse_direction(direction: String) -> (char, i32) {
    let mut chars = direction.chars();
    let dir = chars.next().unwrap();
    let magnitude_str: String = chars.collect();
    let magnitude: i32 = magnitude_str.parse().unwrap();
    (dir, magnitude)
}

fn build_wire(line: &str) -> Vec<Pair> {
    let mut wire: Vec<Pair> = vec![Pair { x: 0, y: 0 }];

    for entry in line.split(",") {
        let last_entry = wire.last().unwrap();
        let (dir, magnitude) = parse_direction(entry.to_owned());
        let mut new_entry = last_entry.clone();
        if dir == 'U' {
            new_entry.y += magnitude;
        }
        if dir == 'R' {
            new_entry.x += magnitude;
        }
        if dir == 'D' {
            new_entry.y -= magnitude;
        }
        if dir == 'L' {
            new_entry.x -= magnitude;
        }
        wire.push(new_entry);
    }

    wire
}

fn is_horizontal(p1: Pair, p2: Pair) -> bool {
    p1.y - p2.y == 0
}

fn is_vertical(p1: Pair, p2: Pair) -> bool {
    p1.x - p2.x == 0
}

fn mix_lines_intersect(h1: Pair, h2: Pair, v1: Pair, v2: Pair) -> bool {
    if !((i32::min(h1.x, h2.x) <= v1.x) && (v1.x <= i32::max(h1.x, h2.x))) {
        return false;
    }
    if !((i32::min(v1.y, v2.y) <= h1.y) && (h1.y <= i32::max(v1.y, v2.y))) {
        return false;
    }
    true
}

fn get_intersection(a1: Pair, a2: Pair, b1: Pair, b2: Pair) -> Option<Pair> {
    let (mut h1, mut h2, mut v1, mut v2) = (b1, b2, a1, a2);
    if is_horizontal(a1, a2) {
        h1 = a1;
        h2 = a2;
        v1 = b1;
        v2 = b2;
    }
    if !mix_lines_intersect(h1, h2, v1, v2) {
        return None;
    }
    Some(Pair { x: v1.x, y: h1.y })
}

struct Intersections {
    wire1: Vec<Pair>,
    wire2: Vec<Pair>,
    index1_save: usize,
    index2_save: usize,
}

impl Intersections {
    fn from_wires(w1: Vec<Pair>, w2: Vec<Pair>) -> Intersections {
        let wire1 = w1.clone();
        let wire2 = w2.clone();
        let index1_save = 1;
        let index2_save = 1;
        Intersections {
            wire1,
            wire2,
            index1_save,
            index2_save,
        }
    }
}

impl Iterator for Intersections {
    type Item = Pair;

    fn next(&mut self) -> Option<Pair> {
        for index1 in self.index1_save..self.wire1.len() {
            let first1 = self.wire1[index1 - 1];
            let second1 = self.wire1[index1];
            for index2 in self.index2_save..self.wire2.len() {
                let first2 = self.wire2[index2 - 1];
                let second2 = self.wire2[index2];
                if let Some(intersection) = get_intersection(first1, second1, first2, second2) {
                    self.index2_save = index2 + 1;
                    return Some(intersection);
                }
                self.index2_save = index2 + 1;
            }
            self.index2_save = 1;
            self.index1_save = index1 + 1;
        }
        None
    }
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let wire1_raw = lines[0];
    let wire2_raw = lines[1];

    let wire1 = build_wire(wire1_raw);
    let wire2 = build_wire(wire2_raw);

    let mut largest = 0;
    for intersection in Intersections::from_wires(wire1, wire2) {
        if intersection.is_zero() {
            continue;
        }
        let distance = id(intersection);
        if largest == 0 || distance < largest {
            largest = distance;
        }
    }
    println!("{}", largest);
}

fn pair_is_between_pairs(a: Pair, b: Pair, p: Pair) -> bool {
    if (a.x == p.x) && (b.x == p.x) {
        return true;
    } else if (a.y == p.y) && (b.y == p.y) {
        return true;
    } else {
        return false;
    }
}

fn get_length_to_intersection(wire: Vec<Pair>, p: Pair) -> i32 {
    let mut sum: i32 = 0;
    for index in 1..wire.len() {
        let a = wire[index - 1];
        let b = wire[index];
        if pair_is_between_pairs(a, b, p) {
            sum += d(a, p);
            break;
        } else {
            sum += d(a, b);
        }
    }
    sum
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let wire1_raw = lines[0];
    let wire2_raw = lines[1];

    let wire1 = build_wire(wire1_raw);
    let wire2 = build_wire(wire2_raw);

    let mut smallest = i32::max_value();
    let mut answer = 0;
    for intersection in Intersections::from_wires(wire1.clone(), wire2.clone()) {
        if intersection.is_zero() {
            continue;
        }
        let t1 = get_length_to_intersection(wire1.clone(), intersection);
        let t2 = get_length_to_intersection(wire2.clone(), intersection);
        let diff = (t1 - t1).abs();
        if diff < smallest {
            smallest = diff;
            answer = t1 + t2;
        }
    }
    println!("{}", answer);
}
