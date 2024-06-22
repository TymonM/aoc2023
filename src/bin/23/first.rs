use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Vertex {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn arrow(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
    fn from_arrow(arrow: char) -> Option<Direction> {
        match arrow {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Vertex {
    pub(crate) fn apply(&self, direction: &Direction) -> Vertex {
        match direction {
            Direction::Up => Vertex {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Vertex {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Vertex {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Vertex {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct Graph {
    pub(crate) incoming: HashMap<Vertex, HashMap<Vertex, usize>>,
    pub(crate) outgoing: HashMap<Vertex, HashMap<Vertex, usize>>,
}

pub fn run(input: &str) {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let starting_space = Vertex {
        row: 0,
        col: input.lines().next().unwrap().find('.').unwrap(),
    };
    let ending_space = Vertex {
        row: map.len() - 1,
        col: input.lines().last().unwrap().find('.').unwrap(),
    };

    let mut graph = Graph {
        incoming: HashMap::new(),
        outgoing: HashMap::new(),
    };
    setup_graph(
        &map,
        starting_space.clone(),
        ending_space.clone(),
        Direction::Down,
        &mut graph,
        true,
    );
    // dbg!(&graph);
    let longest_path =
        find_longest_path(&graph, &starting_space, &ending_space, &mut HashSet::new());
    println!("{longest_path}");
}

fn read_map_at_vertex(map: &[Vec<char>], vertex: &Vertex) -> char {
    map[vertex.row][vertex.col]
}

pub(crate) fn setup_graph(
    map: &[Vec<char>],
    starting_space: Vertex,
    ending_space: Vertex,
    mut from_direction: Direction,
    graph: &mut Graph,
    directed: bool,
) {
    let mut current_space = starting_space.apply(&from_direction);
    let mut distance = 1;
    let mut bidirectional = true;
    loop {
        if directed {
            if let Some(direction) = Direction::from_arrow(read_map_at_vertex(map, &current_space))
            {
                current_space = current_space.apply(&direction);
                bidirectional = false;
                distance += 1;
                continue;
            }
        }
        let mut options = vec![];
        for next_direction in &DIRECTIONS {
            if *next_direction == from_direction.opposite() {
                continue;
            }
            let next_space = current_space.apply(next_direction);
            if next_space == ending_space {
                distance += 1;
                add_edge(&starting_space, &ending_space, graph, distance);
                return;
            }
            let next_space_char = read_map_at_vertex(map, &next_space);
            if next_space_char != '#'
                && !(directed && next_space_char == next_direction.opposite().arrow())
            {
                options.push(next_direction);
            }
        }
        if options.is_empty() {
            return;
        } else if options.len() == 1 {
            from_direction = options[0].clone();
            current_space = current_space.apply(&from_direction);
            distance += 1;
        } else {
            let already_visited =
                current_space == starting_space || graph.outgoing.contains_key(&current_space);
            add_edge(&starting_space, &current_space, graph, distance);
            if !directed || bidirectional {
                add_edge(&current_space, &starting_space, graph, distance);
            }
            if already_visited {
                return;
            }
            for option in options {
                setup_graph(
                    map,
                    current_space.clone(),
                    ending_space.clone(),
                    option.clone(),
                    graph,
                    directed,
                );
            }
        }
    }
}

fn add_edge(starting_space: &Vertex, ending_space: &Vertex, graph: &mut Graph, distance: usize) {
    if let Some(previous) = graph
        .incoming
        .entry(ending_space.clone())
        .or_default()
        .insert(starting_space.clone(), distance)
    {
        graph
            .incoming
            .get_mut(&ending_space)
            .unwrap()
            .insert(starting_space.clone(), usize::max(previous, distance));
    }
    if let Some(previous) = graph
        .outgoing
        .entry(starting_space.clone())
        .or_default()
        .insert(ending_space.clone(), distance)
    {
        graph
            .outgoing
            .get_mut(&starting_space)
            .unwrap()
            .insert(ending_space.clone(), usize::max(previous, distance));
    }
}

pub(crate) fn find_longest_path(
    graph: &Graph,
    starting_space: &Vertex,
    ending_space: &Vertex,
    excluding: &mut HashSet<Vertex>,
) -> usize {
    if starting_space == ending_space {
        return 0;
    }
    let mut longest_path = 0;
    for (from, distance) in graph.incoming.get(ending_space).unwrap() {
        if excluding.contains(from) {
            continue;
        }
        excluding.insert(from.clone());
        let path = distance + find_longest_path(graph, starting_space, from, excluding);
        excluding.remove(from);
        longest_path = usize::max(longest_path, path);
    }
    longest_path
}
