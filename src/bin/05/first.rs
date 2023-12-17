use crate::map::Map;

pub fn run(seeds_line: &str, maps: &[Map]) {
    let seeds = read_seeds(seeds_line);
    let mut min = i64::MAX;
    for seed in seeds {
        let mut previous = seed;
        for map in maps.iter() {
            previous = map.map(previous);
        }
        if previous < min {
            min = previous;
        }
    }
    println!("{}", min);
}

fn read_seeds(seeds_input: &str) -> Vec<i64> {
    let splits = seeds_input.split(' ').collect::<Vec<&str>>();
    let splits = &splits[1..];

    splits
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}
