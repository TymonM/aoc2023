mod first;
mod second;

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/bin/02/input.txt").expect("input.txt? :raise-eyebrow:");

    first::run(input.clone());
    second::run(input.clone());
}