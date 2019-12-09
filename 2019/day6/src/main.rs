use petgraph::algo;
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph, Undirected};
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn count_orbits(depth: i32, node: NodeIndex, graph: &Graph<&str, &str>) -> i32 {
    let mut sum = 0;
    let neighbors: Vec<NodeIndex> = graph
        .neighbors_directed(node, Direction::Outgoing)
        .collect();
    if neighbors.is_empty() {
        return depth;
    }
    for neighbor in neighbors {
        sum += count_orbits(depth + 1, neighbor, graph);
    }
    sum + depth
}

fn build_graph(input: &str) -> Graph<&str, &str> {
    let mut graph = Graph::<&str, &str>::new();
    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();
    for line in input.lines() {
        let objects: Vec<&str> = line.split(')').collect();
        for index in 0..2 {
            let word = objects[index].trim();
            if !node_map.contains_key(word) {
                let node = graph.add_node(word);
                node_map.insert(word, node);
            }
        }
    }
    for line in input.lines() {
        let objects: Vec<&str> = line.split(')').collect();
        let node1 = node_map.get(objects[0].trim()).unwrap();
        let node2 = node_map.get(objects[1].trim()).unwrap();
        graph.extend_with_edges(&[(node1.clone(), node2.clone())]);
    }

    graph
}

fn part1(input: &str) {
    let graph = build_graph(input);
    let externals: Vec<NodeIndex> = graph.externals(Direction::Incoming).collect();
    let mut sum = 0;
    for ext in externals.clone() {
        sum += count_orbits(0, ext, &graph);
    }
    println!("{}", sum);
}

fn part2(input: &str) {
    let mut graph: Graph<&str, &str, Undirected> = build_graph(input).into_edge_type();

    let mut santa = NodeIndex::<u32>::new(0);
    let mut you = NodeIndex::<u32>::new(0);

    for (index, weight) in graph.node_weights_mut().enumerate() {
        if weight == &"SAN" {
            santa = NodeIndex::<u32>::new(index);
        }
        if weight == &"YOU" {
            you = NodeIndex::<u32>::new(index);
        }
    }

    println!("santa: {:?}, you: {:?}", santa, you);

    let path = algo::astar(&graph, you, |n| n == santa, |_| 1, |_| 0);

    match path {
        Some((cost, _)) => {
            println!("{}", cost - 2);
        }
        None => println!("No path available"),
    }
}
