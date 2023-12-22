use std::fs::read_to_string;

mod first;
mod pulse;
mod second;

fn main() {
    let input = read_to_string("src/bin/20/input.txt").expect("input.txt? :raise-eyebrow:");

    first::run(&input);
    second::run(&input);
}
