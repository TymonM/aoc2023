use itertools::Itertools;

pub fn run(input: &str) {
    let mut current_height = 0;
    let mut area = 0;
    for line in input.lines() {
        let (mut direction, steps) = line.split_whitespace().next_tuple().unwrap();
        let steps = steps.parse::<isize>().unwrap();
        match direction {
            "U" => current_height += steps,
            "D" => {
                area += steps;
                current_height -= steps;
            }
            "R" => area += steps * current_height,
            "L" => area -= steps * (current_height - 1),
            _ => panic!("Unexpected char (direction) in input `{}`", direction),
        }
    }
    area += 1;
    println!("{}", area);
}
