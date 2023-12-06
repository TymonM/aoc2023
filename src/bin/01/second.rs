use std::fs::read_to_string;

pub fn run() {
    let mut sum = 0;

    let input = read_to_string("src/bin/01/input.txt").unwrap();
    for line in input.lines() {
        let mut calibration_value = String::from("");
        let mut substr = String::from("");
        let mut i = 0;
        loop {
            substr.push(line.chars().nth(i).unwrap());
            match ends_in_num(substr.clone(), false) {
                Some(x) => {
                    calibration_value.push(x);
                    break;
                },
                None => {},
            }
            i += 1;
        }
        i = line.len() - 1;
        substr = String::from("");
        loop {
            substr.push(line.chars().nth(i).unwrap());
            match ends_in_num(substr.clone(), true) {
                Some(x) => {
                    calibration_value.push(x);
                    break;
                },
                None => {},
            }
            i -= 1;
        }
        sum += calibration_value.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

// Oh wait, I could have done this by just replacing with digits and calling first_part()
fn ends_in_num(str : String, rev : bool) -> Option<char> {
    if str.len() >= 3 && (&str[str.len()-3..] == "one"
        || rev && &str[str.len()-3..] == "eno") {
        return Some('1');
    }
    if str.len() >= 3 && (&str[str.len()-3..] == "two"
        || rev && &str[str.len()-3..] == "owt")  {
        return Some('2');
    }
    if str.len() >= 5 && (&str[str.len()-5..] == "three"
        || rev && &str[str.len()-5..] == "eerht")  {
        return Some('3');
    }
    if str.len() >= 4 && (&str[str.len()-4..] == "four"
        || rev && &str[str.len()-4..] == "ruof")  {
        return Some('4');
    }
    if str.len() >= 4 && (&str[str.len()-4..] == "five"
        || rev && &str[str.len()-4..] == "evif")  {
        return Some('5');
    }
    if str.len() >= 3 && (&str[str.len()-3..] == "six"
        || rev && &str[str.len()-3..] == "xis")  {
        return Some('6');
    }
    if str.len() >= 5 && (&str[str.len()-5..] == "seven"
        || rev && &str[str.len()-5..] == "neves")  {
        return Some('7');
    }
    if str.len() >= 5 && (&str[str.len()-5..] == "eight"
        || rev && &str[str.len()-5..] == "thgie")  {
        return Some('8');
    }
    if str.len() >= 4 && (&str[str.len()-4..] == "nine"
        || rev && &str[str.len()-4..] == "enin")  {
        return Some('9');
    }
    if str.chars().last().unwrap().is_digit(10) {
        return Some(str.chars().last().unwrap());
    }

    None
}