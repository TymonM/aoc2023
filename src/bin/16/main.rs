use std::fs::read_to_string;

mod beam;
mod first;
mod second;

fn main() {
    let input = read_to_string("src/bin/16/input.txt").expect("input.txt? :raise-eyebrow:");

    first::run(&input);
    second::run(&input);
}
