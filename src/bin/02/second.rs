enum Colour {
    Red(u32),
    Green(u32),
    Blue(u32),
}

pub fn run(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut i = 5; // skip the first 5 chars: "Game "
        while line.chars().nth(i).unwrap().is_ascii_digit() {
            i += 1;
        }
        i += 2; // skip the ": "

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        while i < line.len() {
            let (colour, new_index) = read_num_colour(line.to_string(), i);
            i = new_index + 2; // skip delimiter

            match colour {
                Colour::Red(count) => {
                    if count > max_r {
                        max_r = count;
                    }
                }
                Colour::Green(count) => {
                    if count > max_g {
                        max_g = count;
                    }
                }
                Colour::Blue(count) => {
                    if count > max_b {
                        max_b = count;
                    }
                }
            }
        }
        sum += max_r * max_g * max_b;
    }

    println!("{}", sum);
}

fn read_num_colour(str: String, mut i: usize) -> (Colour, usize) {
    let mut count = String::from("");
    while str.chars().nth(i).unwrap().is_ascii_digit() {
        count.push(str.chars().nth(i).unwrap());
        i += 1;
    }
    let count = count.parse().unwrap();
    i += 1; // skip space

    match str.chars().nth(i).unwrap() {
        'r' => (Colour::Red(count), i + 3),
        'g' => (Colour::Green(count), i + 5),
        'b' => (Colour::Blue(count), i + 4),
        _ => panic!("ijfLHSDnfuHS<fjalkSJ -- What colour is that!??!?!"),
    }
}
