use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
}

fn part1(input: &str) {
    let mut graph = Graph {
        values: [[0; 26]; 26],
    };
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let parent = words[1].as_bytes()[0] as char;
        let child = words[7].as_bytes()[0] as char;
        graph.set_parent(child, parent);
    }

    let mut answer = String::new();

    while graph.get_parentless().len() > 0 {
        let mut parentless = graph.get_parentless();
        parentless.sort();
        println!("{:?}", parentless);
        let next = parentless[0];
        answer.push(next);
        graph.pop_node(next);
    }
    println!("{}", answer);
}

fn get_index(letter: char) -> u8 {
    return letter as u8 - 65;
}

#[test]
fn test_get_index() {
    assert_eq!(get_index('A'), 0);
    assert_eq!(get_index('B'), 1);
    assert_eq!(get_index('Z'), 25);
}

fn get_letter(value: u8) -> char {
    return (value + 65) as char;
}

#[test]
fn test_get_letter() {
    assert_eq!(get_letter(0), 'A');
    assert_eq!(get_letter(1), 'B');
    assert_eq!(get_letter(25), 'Z');
}

// Convention is that row denotes children.
// If node is its own child, that just means that
// it exists.
struct Graph {
    values: [[i32; 26]; 26],
}

impl Graph {
    fn exists(&self, letter: char) -> bool {
        let index = get_index(letter) as usize;
        return self.values[index][index] == 1;
    }

    fn set_exists(&mut self, letter: char) {
        let index = get_index(letter) as usize;
        self.values[index][index] = 1;
    }

    fn set_parent(&mut self, child: char, parent: char) {
        self.set_exists(child);
        self.set_exists(parent);
        let child_index = get_index(child) as usize;
        let parent_index = get_index(parent) as usize;

        self.values[parent_index][child_index] = 1;
    }

    fn get_children(&self, parent: char) -> Vec<char> {
        let parent_index = get_index(parent) as usize;
        let mut children: Vec<char> = Vec::new();
        for (index, value) in self.values[parent_index].iter().enumerate() {
            if index == parent_index {
                continue;
            }
            if *value == 1 {
                let child_letter = get_letter(index as u8);
                children.push(child_letter);
            }
        }
        children
    }

    fn has_children(&self, parent: char) -> bool {
        return self.get_children(parent).len() > 0;
    }

    fn get_parents(&self, child: char) -> Vec<char> {
        let child_index = get_index(child) as usize;
        let mut parents: Vec<char> = Vec::new();
        for index in 0..self.values.len() {
            if index == child_index {
                continue;
            }
            let value = self.values[index][child_index];
            if value == 1 {
                let parent_letter = get_letter(index as u8);
                parents.push(parent_letter);
            }
        }
        parents
    }

    fn has_parents(&self, child: char) -> bool {
        return self.get_parents(child).len() > 0;
    }

    fn get_parentless(&self) -> Vec<char> {
        let mut parentless: Vec<char> = Vec::new();
        for index in 0..self.values.len() {
            let letter = get_letter(index as u8);
            if self.exists(letter) && !self.has_parents(letter) {
                parentless.push(letter);
            }
        }
        parentless
    }

    fn pop_node(&mut self, node: char) {
        let index = get_index(node) as usize;
        self.values[index] = [0; 26];
        for row in 0..self.values.len() {
            self.values[row][index] = 0;
        }
    }
}
