// reference images:
// https://web.goodnotes.com/s/GPljYjxaym9XGTVdUvbwjR#page-22
// and
// https://www.reddit.com/r/adventofcode/comments/18o4y0m/2023_day_21_part_2_algebraic_solution_using_only/#lightbox

use crate::first;

const STEPS: usize = 26501365;

pub fn run(input: &str) {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let start_coords = map
        .iter()
        .enumerate()
        .map(|(y, row)| (row.iter().position(|c| *c == 'S').unwrap_or(0), y))
        .max_by_key(|x| x.0)
        .unwrap();
    let odd_all = first::reachable_spaces(&mut map.clone(), &start_coords, 139);
    let even_all = first::reachable_spaces(&mut map.clone(), &start_coords, 138);
    let odd_parts = first::reachable_spaces(&mut map.clone(), &(0, 0), 195)
        + first::reachable_spaces(&mut map.clone(), &(130, 0), 195)
        + first::reachable_spaces(&mut map.clone(), &(0, 130), 195)
        + first::reachable_spaces(&mut map.clone(), &(130, 130), 195);
    let even_parts = first::reachable_spaces(&mut map.clone(), &(0, 0), 64)
        + first::reachable_spaces(&mut map.clone(), &(130, 0), 64)
        + first::reachable_spaces(&mut map.clone(), &(0, 130), 64)
        + first::reachable_spaces(&mut map.clone(), &(130, 130), 64);
    let odd_corners = first::reachable_spaces(&mut map.clone(), &(0, 65), 130)
        + first::reachable_spaces(&mut map.clone(), &(130, 65), 130)
        + first::reachable_spaces(&mut map.clone(), &(65, 0), 130)
        + first::reachable_spaces(&mut map.clone(), &(65, 130), 130);

    let n = (STEPS - 65) / 131;
    let count = (n - 1) * (n - 1) * odd_all
        + n * n * even_all
        + (n - 1) * odd_parts
        + n * even_parts
        + odd_corners;
    println!("{}", count);
}
