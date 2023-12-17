pub fn run(input: &str) {
    let histories = input
        .lines()
        .map(|line| {
            let mut temp = line
                .split_whitespace()
                .map(|val| val.parse::<i64>().expect("not an i64!"))
                .collect::<Vec<i64>>();
            temp.reverse();
            temp
        })
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;
    for history in histories {
        let table = climb_down(history);
        let extrapolated_table = climb_up(table);
        sum += extrapolated_table[0].iter().last().unwrap();
    }

    println!("{}", sum);
}

fn climb_down(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut table = vec![history.clone()];
    let mut previous = history;
    // Lazy way of checking not all equal
    while previous.iter().max().unwrap() != previous.iter().min().unwrap() {
        let mut current = vec![];
        let mut i = 1;
        while i < previous.len() {
            current.push(previous[i] - previous[i - 1]);
            i += 1;
        }
        previous = current;
        table.push(previous.clone());
    }

    table
}

fn climb_up(table: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = table.clone();

    let mut i = table.len();
    let mut previous = 0;
    while i > 0 {
        previous += result[i - 1].last().unwrap();
        result[i - 1].push(previous);
        i -= 1;
    }

    result
}
