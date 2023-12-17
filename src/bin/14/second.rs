use std::collections::HashMap;

pub fn run(input: &str) {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut seen = HashMap::new();
    for i in 0..1000000000 {
        for _ in 0..4 {
            map = roll_up(&map);
            map = rotate(&map);
        }
        if seen.contains_key(&map) {
            if (999999999 - i) % (i - seen[&map]) == 0 {
                println!("{}", calculate_load(&map));
                break;
            }
        } else {
            seen.insert(map.clone(), i);
        }
    }
}

fn roll_up(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut current_heights = vec![0; map[0].len()];
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                'O' => {
                    new_map[current_heights[x]][x] = 'O';
                    current_heights[x] += 1;
                }
                '#' => {
                    new_map[y][x] = '#';
                    current_heights[x] = y + 1;
                }
                '.' => (),
                _ => panic!("Unknown char in input: `{}`", c),
            }
        }
    }

    new_map
}

fn rotate(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rotated_map = vec![vec![' '; map.len()]; map[0].len()];

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            rotated_map[x][map.len() - y - 1] = *c;
        }
    }

    rotated_map
}

fn calculate_load(map: &[Vec<char>]) -> usize {
    let mut load = 0;
    for (y, row) in map.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                load += map.len() - y;
            }
        }
    }

    load
}
