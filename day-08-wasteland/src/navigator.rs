use std::collections::HashMap;

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

    let mut current_node = network.get(&String::from("AAA")).unwrap();
    let mut steps: Vec<char> = vec![];
    while current_node.id != "ZZZ" {
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
