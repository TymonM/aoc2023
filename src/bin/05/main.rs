use std::fs::read_to_string;
use crate::map::Map;

mod map;
mod first;
mod second;

fn main() {
    let input = read_to_string("src/bin/05/input.txt").expect("input.tx? :raise-eyebrow:");
    let seeds_line = input.lines().nth(0).unwrap();
    let maps = Map::input_to_maps(input.as_str());

    first::run(seeds_line, &maps);
    second::run(seeds_line, &maps);
}