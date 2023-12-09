use prime_factorization::Factorization;
use std::{cmp, collections::HashMap};

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

pub fn calculate_steps(map: &str) -> usize {
    let (instructions, network_str) = map.split_once("\n\n").unwrap();

    let network = network_str.split("\n").fold(
        HashMap::new() as HashMap<String, Node>,
        |mut nodes, line| {
            let id = line[..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            let node = Node {
                id: line[..3].to_string(),
                left,
                right,
            };
            nodes.insert(id, node);
            return nodes;
        },
    );

    let mut current_node = network.values().find(node_ends_with('A')).unwrap();
    let mut steps: Vec<char> = vec![];
    while !node_ends_with('Z')(&current_node) {
        for c in instructions.chars() {
            current_node = if c == 'R' {
                network.get(&current_node.right).unwrap()
            } else {
                network.get(&current_node.left).unwrap()
            };
            steps.push(c);
        }
    }

    return steps.len();
}

pub fn calculate_ghost_steps(map: &str) -> u64 {
    let (instructions, network_str) = map.split_once("\n\n").unwrap();

    let network = network_str.split("\n").fold(
        HashMap::new() as HashMap<String, Node>,
        |mut nodes, line| {
            let id = line[..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            let node = Node {
                id: line[..3].to_string(),
                left,
                right,
            };
            nodes.insert(id, node);
            return nodes;
        },
    );

    let mut nodes_steps: Vec<usize> = vec![];
    for node in network.values().filter(node_ends_with('A')) {
        let mut current_node = node;
        let mut steps: usize = 0;
        while !node_ends_with('Z')(&current_node) {
            for c in instructions.chars() {
                if c == 'R' {
                    current_node = network.get(&current_node.right).unwrap();
                } else {
                    current_node = network.get(&current_node.left).unwrap();
                }
            }
            steps += instructions.len();
        }
        nodes_steps.push(steps);
    }

    let fac = nodes_steps
        .iter()
        .map(|num| Factorization::run(*num as u64))
        .collect::<Vec<Factorization<u64>>>();

    return least_common_multiple(fac) as u64;
}

fn node_ends_with(c: char) -> Box<dyn Fn(&&Node) -> bool> {
    Box::new(move |node: &&Node| node.id.chars().nth(2).unwrap() == c)
}

fn least_common_multiple(nums: Vec<Factorization<u64>>) -> u64 {
    let mut factors: HashMap<u64, usize> = HashMap::new();
    for num in nums {
        let mut num_factors: HashMap<u64, usize> = HashMap::new();
        for factor in num.factors {
            if num_factors.contains_key(&factor) {
                *num_factors.get_mut(&factor).unwrap() += 1;
            } else {
                num_factors.insert(factor, 1);
            }
        }
        for (factor, multiple) in num_factors {
            if factors.contains_key(&factor) {
                *factors.get_mut(&factor).unwrap() = cmp::max(factors[&factor], multiple);
            } else {
                factors.insert(factor, multiple);
            }
        }
    }
    return factors.iter().fold(1_u64, |acc, (key, multiple)| {
        acc * key.pow(*multiple as u32)
    });
}
