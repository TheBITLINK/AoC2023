use num::integer::lcm;
use std::fs::File;
use std::io::{self, BufRead};

fn part1(steps: &Vec<char>, nodes: &Vec<Node>) -> usize {
    let mut step_counter = 0;
    let mut current_node = nodes.iter().find(|node| node.id == "AAA").unwrap();
    while current_node.id != "ZZZ" {
        let next_id = if steps[step_counter % steps.len()] == 'L' {
            &current_node.left
        } else {
            &current_node.right
        };
        current_node = nodes.iter().find(|node| node.id == *next_id).unwrap();
        step_counter += 1;
    }
    step_counter
}

fn part2(steps: &Vec<char>, nodes: &Vec<Node>) -> usize {
    let starting_nodes = nodes.iter().filter(|node| node.id.ends_with("A"));
    starting_nodes
        .map(|node| {
            let mut step_counter = 0;
            let mut current_node = node;
            while !current_node.id.ends_with("Z") {
                let next_id = if steps[step_counter % steps.len()] == 'L' {
                    &current_node.left
                } else {
                    &current_node.right
                };
                current_node = nodes.iter().find(|node| node.id == *next_id).unwrap();
                step_counter += 1;
            }
            step_counter
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap()
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let steps: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut nodes: Vec<Node> = Vec::new();
    for line in lines {
        let definition = line.unwrap();
        if definition.len() > 0 {
            nodes.push(Node::from_str(&definition));
        }
    }

    println!("Result for part 1: {}", part1(&steps, &nodes));
    println!("Result for part 2: {}", part2(&steps, &nodes));
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn from_str(input: &str) -> Self {
        Self {
            id: input[0..3].to_string(),
            left: input[7..10].to_string(),
            right: input[12..15].to_string(),
        }
    }
}
