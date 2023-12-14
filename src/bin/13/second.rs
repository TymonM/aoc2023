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

    for pattern in patterns {
        let smudged_patterns = all_smudges(&pattern);
        let initial_mirror = find_mirror(&pattern, 0).unwrap();
        for smudged in &smudged_patterns {
            if let Some(mirror) = find_mirror(smudged, initial_mirror){
                sum += mirror;
                break;
            }
        }
    }
    println!("{}", sum);
}

fn find_mirror(pattern: &Vec<Vec<char>>, forbidden: usize) -> Option<usize> {
    let mut x = 1;
    while x < pattern[0].len() {
        if x != forbidden && does_mirror_vertical(&pattern, x) {
            return Some(x);
        }
        x += 1;
    }
    let mut y = 1;
    while y < pattern.len() {
        if 100*y != forbidden && does_mirror_horizontal(&pattern, y) {
            return Some(100 * y);
        }
        y += 1;
    }

    None
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

fn all_smudges(pattern: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut smudged_patterns = vec![];
    let mut pattern = pattern.clone();
    let mut y = 0;
    while y < pattern.len() {
        let mut x = 0;
        while x < pattern[y].len() {
            smudge(&mut pattern, x, y);
            smudged_patterns.push(pattern.clone());
            smudge(&mut pattern, x, y); // unsmudge
            x += 1;
        }
        y += 1;
    }
    smudged_patterns
}

fn smudge(pattern: &mut Vec<Vec<char>>, x: usize, y: usize) {
    pattern[y][x] = match pattern[y][x] {
        '.' => '#',
        '#' => '.',
        _ => panic!("unknown char in input")
    }
}