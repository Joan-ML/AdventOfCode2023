use crate::days::file_to_lines;

#[derive(Debug, Default)]
struct Combination {
    red: u32,
    green: u32,
    blue: u32,
}

impl Combination {
    pub fn assign_red(&mut self, n: u32) {
        self.red = n;
    }

    pub fn assign_green(&mut self, n: u32) {
        self.green = n;
    }

    pub fn assign_blue(&mut self, n: u32) {
        self.blue = n;
    }
    
    pub fn assign_from_string(&mut self, colour: &str, number: u32) {
        match colour {
            "red" => self.assign_red(number),
            "green" => self.assign_green(number),
            "blue" => self.assign_blue(number),
            _ => panic!("Invalid colour!"),
        }
    }

    pub fn is_possible(&self) -> bool {
        if self.red <= 12 && 
            self.green <= 13 &&
            self.blue <= 14 {
            true
        } else {
            false
        }
    }
}

// Sorry to those who might read this

fn id_of_possible_game(line: &str) -> Option<u32> {
    #[derive(Debug)]
    enum CurrentlyParsing {
        Header,
        Id,
        SpaceBeforeNumber,
        Number,
        SpaceBeforeLetter,
        Colour,
    }
    let mut status = CurrentlyParsing::Header;
    let mut id_as_string = String::default();
    let mut current_number: u32 = 0;
    let mut current_number_as_string = String::default();
    let mut current_colour = String::default();
    let mut current_combination = Combination::default();
    let mut id: u32 = 0;
    for char in line.chars() {
        if char == ' '{
            match &status {
                CurrentlyParsing::Header => status = CurrentlyParsing::Id,
                CurrentlyParsing::SpaceBeforeNumber => status = CurrentlyParsing::Number,
                CurrentlyParsing::Number => {
                    status = CurrentlyParsing::SpaceBeforeLetter;
                    current_number = current_number_as_string.parse().unwrap();
                },
                CurrentlyParsing::SpaceBeforeLetter => status = CurrentlyParsing::Colour,
                _ => (),
            }
        } else if char == ':' {
            status = CurrentlyParsing::SpaceBeforeNumber;
            id = id_as_string.parse().unwrap();
        } else if  char == ',' {
            current_combination.assign_from_string(&current_colour, current_number);
            status = CurrentlyParsing::SpaceBeforeNumber;
            current_colour = String::default();
            current_number_as_string = String::default();
        } else if char == ';' {
            current_combination.assign_from_string(&current_colour, current_number);
            if !current_combination.is_possible() {
                return None;
            }
            current_colour = String::default();
            current_number_as_string = String::default();
            current_combination = Combination::default();
            status = CurrentlyParsing::SpaceBeforeNumber;
        } else {
            match &status {
                CurrentlyParsing::Id => id_as_string.push(char),
                CurrentlyParsing::Number => current_number_as_string.push(char),
                CurrentlyParsing::SpaceBeforeLetter => {
                    current_colour.push(char);
                    status = CurrentlyParsing::Colour;
                }
                CurrentlyParsing::Colour => current_colour.push(char),
                _ => (),
            }
        }
    }
    match current_colour.as_str() {
        "red" => current_combination.assign_red(current_number),
        "green" => current_combination.assign_green(current_number),
        "blue" => current_combination.assign_blue(current_number),
        _ => panic!("Invalid colour!"),
    }
    if !current_combination.is_possible() {
        return None;
    }
    Some(id)
}

fn power_of_game(line: &str) -> u32 {
    #[derive(Debug)]
    enum CurrentlyParsing {
        Header,
        Id,
        SpaceBeforeNumber,
        Number,
        SpaceBeforeLetter,
        Colour,
    }
    let mut status = CurrentlyParsing::Header;
    let mut id_as_string = String::default();
    let mut current_number: u32 = 0;
    let mut current_number_as_string = String::default();
    let mut current_colour = String::default();
    let mut current_combination = Combination::default();
    let mut max_cubes: [u32; 3] = [0, 0, 0];
    for char in line.chars() {
        if char == ' '{
            match &status {
                CurrentlyParsing::Header => status = CurrentlyParsing::Id,
                CurrentlyParsing::SpaceBeforeNumber => status = CurrentlyParsing::Number,
                CurrentlyParsing::Number => {
                    status = CurrentlyParsing::SpaceBeforeLetter;
                    current_number = current_number_as_string.parse().unwrap();
                },
                CurrentlyParsing::SpaceBeforeLetter => status = CurrentlyParsing::Colour,
                _ => (),
            }
        } else if char == ':' {
            status = CurrentlyParsing::SpaceBeforeNumber;
        } else if  char == ',' {
            current_combination.assign_from_string(&current_colour, current_number);
            status = CurrentlyParsing::SpaceBeforeNumber;
            current_colour = String::default();
            current_number_as_string = String::default();
        } else if char == ';' {
            current_combination.assign_from_string(&current_colour, current_number);
            if current_combination.red > max_cubes[0] { max_cubes[0] = current_combination.red }
            if current_combination.green > max_cubes[1] { max_cubes[1] = current_combination.green }
            if current_combination.blue > max_cubes[2] { max_cubes[2] = current_combination.blue }
            current_colour = String::default();
            current_number_as_string = String::default();
            current_combination = Combination::default();
            status = CurrentlyParsing::SpaceBeforeNumber;
        } else {
            match &status {
                CurrentlyParsing::Id => id_as_string.push(char),
                CurrentlyParsing::Number => current_number_as_string.push(char),
                CurrentlyParsing::SpaceBeforeLetter => {
                    current_colour.push(char);
                    status = CurrentlyParsing::Colour;
                }
                CurrentlyParsing::Colour => current_colour.push(char),
                _ => (),
            }
        }
    }
    current_combination.assign_from_string(&current_colour, current_number);
    if current_combination.red > max_cubes[0] { max_cubes[0] = current_combination.red }
    if current_combination.green > max_cubes[1] { max_cubes[1] = current_combination.green }
    if current_combination.blue > max_cubes[2] { max_cubes[2] = current_combination.blue }
    return max_cubes[0] * max_cubes[1] * max_cubes[2]
}

pub fn day_02_first(path: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in file_to_lines(path) {
        match id_of_possible_game(&line.expect("Can't read line from input!")) {
            Some(i) => sum += i,
            None => (),
        }
    }
    sum
}

pub fn day_02_second(path: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in file_to_lines(path) {
        sum += power_of_game(&line.expect("Can't read line from input!"));
    }
    sum
}
