pub fn run(input: Vec<Vec<char>>) {
    let mut sum = 0;
    let mut y = 0;
    while y < input.len() {
        let line = &input[y];
        let mut x = 0;
        while x < line.len() {
            if !line[x].is_ascii_digit() {
                x += 1;
                continue;
            }
            let mut part_number = String::from("");
            while x < line.len() && line[x].is_ascii_digit() {
                part_number.push(line[x]);
                x += 1;
            }
            if check_surrounding_rect(x - part_number.len(), y, part_number.len(), &input) {
                sum += part_number.parse::<u32>().unwrap();
            }
        }

        y += 1;
    }

    println!("{}", sum);
}

fn check_surrounding_rect(x: usize, y: usize, len: usize, mat: &Vec<Vec<char>>) -> bool {
    let mut j = if y > 0 {y - 1} else {0};
    while j < mat.len() && j < y + 2 {
        let mut i = if x > 0 {x - 1} else {0};
        while i < mat[j].len() && i < x + len + 1 {
            let c = mat[j][i];
            if c != '.' && !c.is_ascii_digit() {
                return true;
            }
            i += 1;
        }
        j += 1;
    }

    false
}