use std::collections::HashMap;

use crate::node::Node;

pub fn run(input: &str) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let node_defs = lines.skip(1).collect::<Vec<&str>>();

    let mut node_map = HashMap::new();
    for def in node_defs.iter() {
        let label = &def[..3];
        let left = String::from(&def[7..10]);
        let right = String::from(&def[12..15]);

        node_map.insert(label, Node{left, right});
    }

    let mut current_node = "AAA";
    let mut steps = 0;
    'navigate: loop {
        for instruction in instructions.chars() {
            steps += 1;
            if instruction == 'R' {
                current_node = &node_map[current_node].right;
            } else if instruction == 'L' {
                current_node = &node_map[current_node].left;
            }
            if current_node == "ZZZ" {
                break 'navigate;
            }
        }
    }

    println!("{}", steps);
}