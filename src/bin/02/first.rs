pub fn run(input : String) -> () {
    let mut sum = 0;
    let mut game_number = 0;
    'line: for line in input.lines() {
        let mut i = 5; // skip the first 5 chars: "Game "
        while line.chars().nth(i).unwrap().is_ascii_digit() {
            i += 1;
        }
        game_number += 1;
        i += 2; // skip the ": "
        while i < line.len() {
            let (legal, new_index) = read_num_colour(line.to_string(), i);
            if !legal { continue 'line; }
            i = new_index + 2; // skip delimiter
        }
        sum += game_number;
    }

    println!("{}", sum);
}

fn read_num_colour(str: String, mut i: usize) -> (bool, usize) {
    let red_threshold = 12;
    let green_threshold = 13;
    let blue_threshold = 14;

    let mut count = String::from("");
    while str.chars().nth(i).unwrap().is_ascii_digit() {
        count.push(str.chars().nth(i).unwrap());
        i += 1;
    }
    let count = count.parse::<i32>().unwrap();
    i += 1; // skip space

    match str.chars().nth(i).unwrap() {
        'r' => (count <= red_threshold, i+3),
        'g' => (count <= green_threshold, i+5),
        'b' => (count <= blue_threshold, i+4),
        _ => panic!("ijfLHSDnfuHS<fjalkSJ -- What colour is that!??!?!")
    }
}
