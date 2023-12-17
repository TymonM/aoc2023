use crate::map::Map;
use std::fs::{self, read_to_string, File};

const MAPSUM_PATH: &str = "src/bin/05/mapsum.txt";

pub fn run(seeds_line: &str, maps: &Vec<Map>) {
    if File::open(MAPSUM_PATH).is_err() {
        generate_map_sum(seeds_line, maps);
    }
    let map_sum = read_to_string(MAPSUM_PATH).unwrap();

    let seeds = extract_seed_ranges(&seeds_line[7..]);
    let map = &Map::input_to_maps(map_sum.as_str())[0];

    let critical_seeds = Map::critical_intersections(&seeds, map);
    let mut min = i64::MAX;
    let mut minimising_seed = 0;
    for seed in critical_seeds {
        let value = map.map(seed);
        if value < min {
            min = value;
            minimising_seed = seed;
        }
    }

    println!("{} with seed {}", min, minimising_seed);
}

fn generate_map_sum(seeds_line: &str, maps: &Vec<Map>) {
    let mut output = String::from(seeds_line);
    output += "\n\nmap:";
    let mut sum = maps[0].clone();

    let mut i = 1;
    while i < maps.len() {
        sum = Map::add(&sum, &maps[i]);
        i += 1;
    }

    output += sum.print().as_str();
    fs::write(MAPSUM_PATH, output.as_str()).expect("couldn't write to mapsum");
}

fn extract_seed_ranges(seeds_input: &str) -> Map {
    let splits = seeds_input
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut ranges = Map::new();

    let mut i = 0;
    while i < splits.len() {
        ranges.insert(splits[i], splits[i], splits[i + 1]);
        i += 2;
    }

    ranges
}
