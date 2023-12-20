pub fn run(input: &str) {
    let mut current_height = 0;
    let mut area = 0;
    let mut instructions = vec![];
    for line in input.lines() {
        let code = line.split_whitespace().nth(2).unwrap();
        let steps = i64::from_str_radix(&code[2..7], 16).unwrap();
        let direction =
            ['R', 'D', 'L', 'U'][code.chars().nth(7).unwrap().to_digit(10).unwrap() as usize];
        instructions.push((direction, steps));
    }
    for (direction, steps) in instructions {
        match direction {
            'U' => current_height += steps,
            'D' => {
                area += steps;
                current_height -= steps;
            }
            'R' => area += steps * current_height,
            'L' => area -= steps * (current_height - 1),
            _ => unreachable!(),
        }
    }
    area += 1;
    println!("{}", area);
}
