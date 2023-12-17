use std::cmp::{max, min};

const EMPTY_SIZE: usize = 1000000;

pub fn run(input: &str) {
    let mut star_locations = vec![];
    let mut is_empty_row = vec![true; input.len()];
    let mut is_empty_col = vec![true; input.lines().next().unwrap().len()];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    star_locations.push((x, y));
                    is_empty_row[y] = false;
                    is_empty_col[x] = false;
                }
                '.' => (),
                _ => panic!("Unknown char in input!"),
            }
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < star_locations.len() {
        let mut j = i + 1;
        while j < star_locations.len() {
            sum += calculate_distance(
                star_locations[i],
                star_locations[j],
                &is_empty_row,
                &is_empty_col,
            );
            j += 1;
        }
        i += 1;
    }

    println!("{}", sum);
}

fn calculate_distance(
    a: (usize, usize),
    b: (usize, usize),
    is_empty_row: &[bool],
    is_empty_col: &[bool],
) -> usize {
    let left_col = min(a.0, b.0);
    let right_col = max(a.0, b.0);
    let top_row = min(a.1, b.1);
    let bottom_row = max(a.1, b.1);
    let mut distance = right_col - left_col + bottom_row - top_row;

    let mut i = left_col + 1;
    while i < right_col {
        if is_empty_col[i] {
            distance += EMPTY_SIZE - 1;
        }
        i += 1;
    }

    let mut i = top_row + 1;
    while i < bottom_row {
        if is_empty_row[i] {
            distance += EMPTY_SIZE - 1;
        }
        i += 1;
    }

    distance
}
