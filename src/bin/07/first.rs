use crate::hand::Hand;

pub fn run(input: &str) {
    let mut hands = vec![];
    for line in input.lines() {
        let splits = line.split(' ').collect::<Vec<&str>>();
        let hand = Hand::from(splits[0], splits[1], false);
        hands.push(hand);
    }

    // radix sort kinda
    let mut i = 5;
    while i > 0 {
        let mut groups: Vec<Vec<Hand>> = vec![vec![]; 13];
        for hand in hands {
            groups[hand.nth_value(i-1) - 1].push(hand);
        }
        hands = groups.into_iter().flatten().collect();
        i -= 1;
    }

    let mut groups: Vec<Vec<Hand>> = vec![vec![]; 7];
    for hand in hands {
        groups[hand.get_power()].push(hand);
    }
    hands = groups.into_iter().flatten().collect();

    let mut sum = 0;
    let mut i = 0;
    while i < hands.len() {
        sum += (i + 1) as i32 * hands[i].get_bet();
        i += 1;
    }

    println!("{}", sum);
}