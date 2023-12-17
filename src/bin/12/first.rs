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
        sum += ways_to_match_records_efficient(&mut status_map, &records);
        // println!(
        //     "{}: {} ways",
        //     line,
        //     ways_to_match_records_efficient(&mut status_map, &records)
        // );
    }
    println!("{}", sum);
}

fn ways_to_match_records_efficient(status_map: &mut Vec<Status>, records: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut block_index = 0;
    let mut block_size = 0;
    while i < status_map.len() {
        match status_map[i] {
            Status::Working => {
                if block_size != 0 {
                    if block_size != records[block_index] {
                        return 0;
                    }
                    block_index += 1;
                    block_size = 0;
                }
            }
            Status::Broken => {
                block_size += 1;
                if block_index >= records.len() || block_size > records[block_index] {
                    return 0;
                }
            }
            Status::Unknown => break,
        }
        i += 1;
    }
    if i < status_map.len() {
        let original_status = status_map[i];
        status_map[i] = Status::Working;
        let working_ways = ways_to_match_records_efficient(status_map, records);
        status_map[i] = Status::Broken;
        let broken_ways = ways_to_match_records_efficient(status_map, records);
        status_map[i] = original_status;
        return working_ways + broken_ways;
    } else if block_index < records.len() - 1
        || (block_index == records.len() - 1 && block_size != *records.last().unwrap())
    {
        return 0;
    }

    1
}

// Original, less efficient way
#[allow(dead_code)]
fn ways_to_match_records(mut status_map: Vec<Status>, records: &Vec<usize>) -> usize {
    let i = status_map.iter().position(|x| matches!(x, Status::Unknown));
    if let Some(i) = i {
        status_map[i] = Status::Working;
        let working_ways = ways_to_match_records(status_map.clone(), records);
        status_map[i] = Status::Broken;
        let broken_ways = ways_to_match_records(status_map.clone(), records);
        return working_ways + broken_ways;
    }

    if calculate_records(&status_map) == *records {
        1
    } else {
        0
    }
}

fn calculate_records(status_map: &Vec<Status>) -> Vec<usize> {
    let mut records = vec![];
    let mut i = 0;
    while i < status_map.len() {
        let mut block_size = 0;
        while i < status_map.len() {
            if let Status::Broken = status_map[i] {
                block_size += 1;
                i += 1;
            } else {
                break;
            }
        }
        if block_size != 0 {
            records.push(block_size);
        }
        i += 1;
    }

    records
}
