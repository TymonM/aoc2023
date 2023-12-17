#[derive(Copy, Clone)]
pub struct Hand {
    cards: [Card; 5],
    bet: i32,
}

#[derive(Copy, Clone)]
struct Card {
    value: usize,
}

impl Hand {
    pub fn from(cards: &str, bet: &str, joker: bool) -> Hand {
        let mut hand = Hand {
            cards: [Card::new(); 5],
            bet: bet.parse().unwrap(),
        };
        let mut i = 0;
        while i < 5 {
            hand.cards[i].set_value(cards.chars().nth(i).unwrap(), joker);
            i += 1;
        }

        hand
    }

    pub fn get_bet(&self) -> i32 {
        self.bet
    }
    pub fn nth_value(&self, n: usize) -> usize {
        self.cards[n].get_value()
    }

    pub fn get_power(&self) -> usize {
        let mut counts = [0; 13];
        let mut jokers = 0;
        for card in self.cards {
            if card.get_value() == 0 {
                jokers += 1;
                continue;
            }
            counts[card.get_value() - 1] += 1;
        }

        let mut max_count = 0;
        let mut i = 0;
        while i < counts.len() {
            if counts[i] > counts[max_count] {
                max_count = i;
            }
            i += 1;
        }
        counts[max_count] += jokers;

        let mut triple = false;
        let mut doubles = 0;
        for count in counts {
            match count {
                5 => return 6, // 5 of a kind
                4 => return 5, // 4 of a kind
                3 => triple = true,
                2 => doubles += 1,
                _ => (),
            }
        }
        if triple {
            return if doubles == 1 {
                4 // full house
            } else {
                3 // three of a kind
            };
        }

        doubles // nasty, covers remaining cases
    }
}

impl Card {
    pub fn new() -> Card {
        Card { value: 0 }
    }

    pub fn set_value(&mut self, symbol: char, joker: bool) {
        match symbol {
            'T' => self.value = 9,
            'J' => self.value = if joker { 0 } else { 10 },
            'Q' => self.value = 11,
            'K' => self.value = 12,
            'A' => self.value = 13,
            num => {
                self.value = num
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("Unknown symbol `{}`", num))
                    as usize
                    - 1
            }
        }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }
}

// printing for debugging only
#[allow(dead_code)]
impl Hand {
    pub fn print(&self) -> String {
        let mut output = String::from("");
        for card in self.cards {
            output.push(card.print());
        }
        output.push(' ');
        output += &self.bet.to_string();

        output
    }
}
#[allow(dead_code)]
impl Card {
    pub fn print(&self) -> char {
        "J23456789TJQKA".as_bytes()[self.value] as char
    }
}
