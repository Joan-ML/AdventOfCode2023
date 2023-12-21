use crate::days::file_to_lines;

fn add_first_and_last(line: &str) -> u32 {
    let mut digits = ['0', '0'];
    let mut found_first = false;
    for char in line.chars() {
        if char.is_ascii_digit() {
            if !found_first {
                found_first = true;
                digits[0] = char;
                digits[1] = char;
            } else {
                digits[1] = char
            }
        }
    }
    digits.iter().collect::<String>().parse::<u32>().unwrap()
}

fn spelling_to_number(spelling: &str) -> Option<char> {
    println!("{}", spelling);
    match spelling {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn add_first_and_last_including_spelled(line: &str) -> u32 {
    let line = line.chars().collect::<Vec<char>>();
    let length = line.len();
    let mut digits = ['0', '0'];
    let mut found_first = false;
    for (i, char) in line.iter().enumerate() {
        if char.is_ascii_digit() {
            if !found_first {
                found_first = true;
                digits[0] = *char;
                digits[1] = *char;
            } else {
                digits[1] = *char;
            }
        }
        // Sounds inefficient, but I'm already late and this is day 1
        for j in i..length + 1 {
            let s: String = line[i..j].iter().collect();
            match spelling_to_number(&s) {
                Some(s) => {
                    if !found_first {
                        found_first = true;
                        digits[0] = s;
                        digits[1] = s;
                    } else {
                        digits[1] = s;
                    }
                },
                None => (),
            }
        }
    }
    digits.iter().collect::<String>().parse::<u32>().unwrap()
}

pub fn day_01_first(path: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in file_to_lines(path) {
        sum += add_first_and_last(&line.expect("Can't get line from reading the buffer"));
    }
    sum
}

pub fn day_01_second(path: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in file_to_lines(path) {
        sum += add_first_and_last_including_spelled(&line.expect("Can't get line from reading the buffer"));
    }
    sum
}
