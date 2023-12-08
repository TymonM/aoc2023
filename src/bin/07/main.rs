use std::fs::read_to_string;

mod first;
mod second;
mod hand;

fn main() {
    let input = read_to_string("src/bin/07/input.txt").expect("input.txt? :raise-eyebrow:");

    first::run(&input);
    second::run(&input);
}