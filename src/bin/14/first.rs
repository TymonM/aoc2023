pub fn run(input: &str) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut current_heights = vec![0; width];
    let mut sum = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    sum += height - current_heights[col];
                    current_heights[col] += 1;
                }
                '#' => {
                    current_heights[col] = row + 1;
                }
                '.' => (),
                _ => panic!("Unknown char in input: `{}`", c),
            }
        }
    }

    println!("{}", sum);
}
