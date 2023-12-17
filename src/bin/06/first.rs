pub fn run(input: &str) {
    let mut product = 1;
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut i = 0;
    while i < times.len() {
        // this is one of the craziest magic functions ever
        product *= ((times[i] as f32 * times[i] as f32 / 4f32 - (distances[i] + 1) as f32).sqrt()
            + 0.5f32) as i32
            * 2
            - (times[i] + 1) % 2;
        i += 1;
    }

    println!("{}", product);
}
