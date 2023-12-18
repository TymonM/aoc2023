use std::collections::HashMap;

use crate::node::Node;

pub fn run(input: &str) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let node_defs = lines.skip(1).collect::<Vec<&str>>();

    let mut node_map = HashMap::new();
    let mut starting_nodes = vec![];
    for def in node_defs {
        let label = &def[..3];
        let left = String::from(&def[7..10]);
        let right = String::from(&def[12..15]);

        if label.ends_with('A') {
            starting_nodes.push(label);
        }

        node_map.insert(label, Node { left, right });
    }

    let mut z_locations = vec![];
    for node in starting_nodes {
        let mut traversed: Vec<(&str, usize)> = vec![];
        let mut current_node = node;
        let mut count = 0;
        'navigate: loop {
            for (i, instruction) in instructions.chars().enumerate() {
                if traversed.contains(&(current_node, i)) {
                    break 'navigate;
                }
                traversed.push((current_node, i));
                count += 1;
                if instruction == 'R' {
                    current_node = &node_map[current_node].right;
                } else if instruction == 'L' {
                    current_node = &node_map[current_node].left;
                }
                if current_node.ends_with('Z') {
                    z_locations.push(count);
                }
            }
        }
    }

    println!("{}", lcm(z_locations));
}

fn lcm(vals: Vec<i64>) -> i64 {
    let mut least = 1;
    for val in vals {
        least = num::integer::lcm(least, val);
    }

    least
}
