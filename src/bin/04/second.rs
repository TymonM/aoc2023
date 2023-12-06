pub fn run(input : &str) {
    let mut sum = 0;
    let mut totals = vec![1; input.len()];

    for (card_num, line) in input.lines().enumerate() {
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
                card_sum += 1;
            }
            i += 2;
        }

        let mut i = 1;
        while i <= card_sum {
            totals[card_num + i] += totals[card_num];
            i += 1;
        }
        sum += totals[card_num];
    }

    println!("{}", sum);
}