mod first;
mod second;

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/bin/03/input.txt").expect("input.txt? :raise-eyebrow:");
    let blueprint_mat = string_to_matrix(input);

    first::run(blueprint_mat.clone());
    second::run(blueprint_mat.clone());
}

fn string_to_matrix(str: String) -> Vec<Vec<char>> {
    let mut mat = Vec::new();
    for line in str.lines() {
        let mut v = Vec::new();
        for c in line.chars() {
            v.push(c);
        }
        mat.push(v);
    }

    mat
}