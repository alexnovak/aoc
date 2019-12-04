use std::cmp;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
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

fn get_intersection_distance_horizontal(a1: Pair, a2: Pair, b1: Pair, b2: Pair) -> Option<i32> {
    if a1.y != b1.y && a2.y != b2.y {
        return None;
    }
    let minx = *[a1.x, a2.x, b1.x, b2.x].iter().min().unwrap();
    let maxx = *[a1.x, a2.x, b1.x, b2.x].iter().max().unwrap();
    let mut largest = i32::max_value();
    for iter_x in minx..maxx + 1 {
        let val = id(Pair { x: iter_x, y: a1.y });
        if val < largest {
            largest = val;
        }
    }
    Some(largest)
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

fn lines_intersect(a1: Pair, a2: Pair, b1: Pair, b2: Pair) -> bool {
    if is_horizontal(a1, a2) {
        return mix_lines_intersect(a1, a2, b1, b2);
    } else {
        return mix_lines_intersect(b1, b2, a1, a2);
    }
}

fn get_intersection_distance_vertical(a1: Pair, a2: Pair, b1: Pair, b2: Pair) -> Option<i32> {
    if a1.x != b1.x && a2.x != b2.x {
        return None;
    }
    let miny = *[a1.y, a2.y, b1.y, b2.y].iter().min().unwrap();
    let maxy = *[a1.y, a2.y, b1.y, b2.y].iter().max().unwrap();
    let mut largest = i32::max_value();
    for iter_y in miny..maxy + 1 {
        let val = id(Pair { x: a1.x, y: iter_y });
        if val < largest {
            largest = val;
        }
    }
    Some(largest)
}

fn get_intersection_distance_mixed(h1: Pair, h2: Pair, v1: Pair, v2: Pair) -> Option<i32> {
    if !mix_lines_intersect(h1, h2, v1, v2) {
        return None;
    }
    Some(id(Pair { x: v1.x, y: h1.y }))
}

fn get_intersection_distance(a1: Pair, a2: Pair, b1: Pair, b2: Pair) -> Option<i32> {
    if a1.is_zero() || b1.is_zero() || a2.is_zero() || b2.is_zero() {
        return None;
    }
    if is_horizontal(a1, a2) && is_horizontal(b1, b2) {
        return get_intersection_distance_horizontal(a1, a2, b1, b2);
    }
    if is_vertical(a1, a2) && is_vertical(b1, b2) {
        return get_intersection_distance_vertical(a1, a2, b1, b2);
    }
    if is_horizontal(a1, a2) {
        return get_intersection_distance_mixed(a1, a2, b1, b2);
    } else {
        return get_intersection_distance_mixed(b1, b2, a1, a2);
    }
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let wire1_raw = lines[0];
    let wire2_raw = lines[1];

    let wire1 = build_wire(wire1_raw);
    let wire2 = build_wire(wire2_raw);

    let mut largest = 0;

    for index1 in 1..wire1.len() {
        let first1 = wire1[index1 - 1];
        let second1 = wire1[index1];
        for index2 in 1..wire2.len() {
            let first2 = wire2[index2 - 1];
            let second2 = wire2[index2];
            if let Some(vertex_distance) =
                get_intersection_distance(first1, second1, first2, second2)
            {
                if largest == 0 || vertex_distance < largest {
                    largest = vertex_distance;
                }
            }
        }
    }
    println!("{}", largest);
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let wire1_raw = lines[0];
    let wire2_raw = lines[1];

    let wire1 = build_wire(wire1_raw);
    let wire2 = build_wire(wire2_raw);
}
