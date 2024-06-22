use crate::first;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let starting_space = first::Vertex {
        row: 0,
        col: input.lines().next().unwrap().find('.').unwrap(),
    };
    let ending_space = first::Vertex {
        row: map.len() - 1,
        col: input.lines().last().unwrap().find('.').unwrap(),
    };

    let mut graph = first::Graph {
        incoming: HashMap::new(),
        outgoing: HashMap::new(),
    };
    first::setup_graph(
        &map,
        starting_space.clone(),
        ending_space.clone(),
        first::Direction::Down,
        &mut graph,
        false,
    );
    // dbg!(&graph);
    let longest_path =
        first::find_longest_path(&graph, &starting_space, &ending_space, &mut HashSet::new());
    println!("{longest_path}");
}
