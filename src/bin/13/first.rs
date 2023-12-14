pub fn run(input: &str) {
    let mut sum = 0;
    let mut patterns: Vec<Vec<Vec<char>>> = vec![];
    let mut current_pattern = vec![];
    for line in input.lines() {
        let mut pattern_line = vec![];
        if line.is_empty() {
            patterns.push(current_pattern.clone());
            current_pattern.clear();
        } else {
            for c in line.chars() {
                pattern_line.push(c);
            }
            current_pattern.push(pattern_line);
        }
    }
    patterns.push(current_pattern);

    'patterns: for pattern in patterns {
        let mut x = 1;
        while x < pattern[0].len() {
            if does_mirror_vertical(&pattern, x) {
                sum += x;
                continue 'patterns;
            }
            x += 1;
        }
        let mut y = 1;
        while y < pattern.len() {
            if does_mirror_horizontal(&pattern, y) {
                sum += 100 * y;
                continue 'patterns;
            }
            y += 1;
        }
    }
    println!("{}", sum);
}

fn does_mirror_vertical(pattern: &Vec<Vec<char>>, x: usize) -> bool {
    let mut cols = vec![];
    let mut i = 0;
    while i < pattern[0].len() {
        let mut col = vec![];
        let mut y = 0;
        while y < pattern.len() {
            col.push(pattern[y][i]);
            y += 1;
        }
        cols.push(col);
        i += 1;
    }

    let mut i = 0;
    while i <= x {
        if 2*x-i-1 < pattern[0].len() && cols[i] != cols[2 * x - i - 1] {
            return false;
        }
        i += 1;
    }

    true
}

fn does_mirror_horizontal(pattern: &Vec<Vec<char>>, y: usize) -> bool {
    let mut i = 0;
    while i <= y {
        if 2*y-i-1 < pattern.len() && pattern[i] != pattern[2 * y - i - 1] {
            return false;
        }
        i += 1;
    }

    true
}