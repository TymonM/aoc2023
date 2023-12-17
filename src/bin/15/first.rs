pub fn run(input: &str) {
    let mut sum = 0;
    for sequence in input.split(',') {
        sum += hash(sequence);
    }
    println!("{}", sum);
}

pub fn hash(str: &str) -> usize {
    let mut current_value = 0;
    for c in str.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
