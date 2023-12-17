pub fn run(input: &str) {
    let time = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .join("")
        .parse::<i64>()
        .unwrap();

    println!(
        "{}",
        ((time as f32 * time as f32 / 4f32 - (distance + 1) as f32).sqrt() + 0.5f32) as i64 * 2
            - (time + 1) % 2
    );
}
