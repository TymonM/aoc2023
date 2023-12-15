use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Status {
    Working,
    Broken,
    Unknown,
}

pub fn run(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let mut status_map = vec![];
        let mut i = 0;
        while i < line.len() {
            match line.chars().nth(i).unwrap() {
                '.' => status_map.push(Status::Working),
                '#' => status_map.push(Status::Broken),
                '?' => status_map.push(Status::Unknown),
                ' ' => break,
                _ => panic!("unknown char in input"),
            }
            i += 1;
        }
        let records = line[i + 1..]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut extended_status_map = status_map.clone();
        let mut extended_records = records.clone();
        for _ in 0..4 {
            extended_status_map.push(Status::Unknown);
            extended_status_map.append(&mut status_map.clone());
            extended_records.append(&mut records.clone());
        }
        let mut cached = HashMap::new();
        sum += ways_to_match_records(&extended_status_map, &extended_records, &mut cached);
        // println!("Done: {:?}", records); // Debugging only
    }
    println!("{}", sum);
}

// even more efficient than the 'efficient' one in first.rs
fn ways_to_match_records(
    status_map: &[Status],
    records: &[usize],
    cached: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if cached.contains_key(&(status_map.len(), records.len())) {
        return cached[&(status_map.len(), records.len())];
    }

    let max = if let Some(first_broken) = find_first_broken(status_map) {
        if records.is_empty() || records[0] > status_map.len() {
            return 0;
        }
        if first_broken > status_map.len() - records[0] {
            status_map.len() - records[0]
        } else {
            first_broken
        }
    } else {
        if records.is_empty() {
            return 1;
        } else if records[0] > status_map.len() {
            return 0;
        }
        status_map.len() - records[0]
    };

    let block_size = records[0];
    let mut sum = 0;
    'offsets: for i in 0..=max {
        for j in 0..block_size {
            if let Status::Working = status_map[i + j] {
                continue 'offsets;
            }
        }
        if i + block_size < status_map.len() {
            if let Status::Broken = status_map[i + block_size] {
                continue 'offsets;
            }
        }
        if i + block_size + 1 < status_map.len() {
            sum += ways_to_match_records(&status_map[i + block_size + 1..], &records[1..], cached);
        } else if records.len() == 1 {
            sum += 1;
        }
    }

    cached.insert((status_map.len(), records.len()), sum);

    sum
}

fn find_first_broken(status_map: &[Status]) -> Option<usize> {
    status_map
        .iter()
        .position(|x| if let Status::Broken = x { true } else { false })
}
