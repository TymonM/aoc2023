use std::collections::HashMap;

struct Node {
    heat_loss: usize,
    potential: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

pub fn run(input: &str) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut nodes = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut node_row = vec![];
        for (x, c) in line.chars().enumerate() {
            node_row.push(Node {
                heat_loss: c.to_digit(10).unwrap() as usize,
                potential: width + height - x - y,
            });
        }
        nodes.push(node_row);
    }

    let mut shortest_paths = HashMap::new();
    let mut boundary = vec![((0, 0), Direction::None, 0, 0)];
    let mut current_index = 0;
    let mut current = boundary[current_index];
    while current.0 != (width - 1, height - 1) {
        boundary.remove(current_index);
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if direction == current.1 || direction == current.1.opposite() {
                continue;
            }
            let mut heat_lost = current.2;
            let mut coords = current.0;
            for _ in 0..3 {
                if let Some((x, y)) = apply_direction(direction, coords, width, height) {
                    coords = (x, y);
                    heat_lost += nodes[y][x].heat_loss;
                    if heat_lost
                        < *shortest_paths
                            .get(&(x, y, direction))
                            .unwrap_or(&usize::MAX)
                    {
                        shortest_paths.insert((x, y, direction), heat_lost);
                        boundary.push(((x, y), direction, heat_lost, nodes[y][x].potential));
                    }
                }
            }
        }
        current_index = get_minimum_index(&boundary);
        current = boundary[current_index];
    }

    println!("{}", current.2);
}

fn apply_direction(
    direction: Direction,
    coords: (usize, usize),
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if coords.1 > 0 {
                return Some((coords.0, coords.1 - 1));
            }
        }
        Direction::Down => {
            if coords.1 < height - 1 {
                return Some((coords.0, coords.1 + 1));
            }
        }
        Direction::Left => {
            if coords.0 > 0 {
                return Some((coords.0 - 1, coords.1));
            }
        }
        Direction::Right => {
            if coords.0 < width - 1 {
                return Some((coords.0 + 1, coords.1));
            }
        }
        Direction::None => {
            return None;
        }
    }
    None
}

fn get_minimum_index(nodes: &[((usize, usize), Direction, usize, usize)]) -> usize {
    let mut min_val = usize::MAX;
    let mut min_index = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.2 + node.3 < min_val {
            min_val = node.2 + node.3;
            min_index = i;
        }
    }
    min_index
}
