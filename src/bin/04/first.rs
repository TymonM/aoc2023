pub fn run(input : &str) {
    let mut sum = 0;
    for line in input.lines() {
        let mut card_sum = 0;
        let mut winning_nums = Vec::new();

        let mut i = 9; // skip the "Card   1:"
        while i < line.len() {
            i += 1; // skip the space
            if line.chars().nth(i).unwrap() == '|' {
                i += 1;
                break;
            }

            let mut num = String::from("");
            if line.chars().nth(i).unwrap() != ' ' {
                num.push(line.chars().nth(i).unwrap());
            }
            num.push(line.chars().nth(i + 1).unwrap());
            winning_nums.push(num);
            i += 2;
        }

        while i < line.len() {
            i += 1; // skip the space
            let mut num = String::from("");
            if line.chars().nth(i).unwrap() != ' ' {
                num.push(line.chars().nth(i).unwrap());
            }
            num.push(line.chars().nth(i + 1).unwrap());
            if winning_nums.contains(&num) {
                card_sum = if card_sum == 0 {1} else {2 * card_sum};
            }
            i += 2;
        }

        sum += card_sum;
    }

    println!("{}", sum);
}