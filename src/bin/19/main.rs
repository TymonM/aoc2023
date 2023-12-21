use std::fs::read_to_string;

mod first;
mod part_first;
mod part_second;
mod second;

fn main() {
    let input = read_to_string("src/bin/19/input.txt").expect("input.txt? :raise-eyebrow:");

    first::run(&input);
    second::run(&input);
}
