use std::fs::read_to_string;

pub fn run() {
    let mut sum = 0;

    let input = read_to_string("src/bin/01/input.txt").unwrap();
    for line in input.lines() {
        let mut calibration_value = String::from("");
        let mut i = 0;
        while !line.chars().nth(i).unwrap().is_digit(10) {
            i += 1;
        }
        calibration_value.push(line.chars().nth(i).unwrap());
        let mut i = line.len() - 1;
        while !line.chars().nth(i).unwrap().is_digit(10) {
            i -= 1;
        }
        calibration_value.push(line.chars().nth(i).unwrap());
        sum += calibration_value.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}