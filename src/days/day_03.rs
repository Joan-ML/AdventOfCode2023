use crate::days::file_to_lines;

fn construct_character_matrix(path: &str) -> Vec<Vec<char>> {
    let mut all_lines: Vec<Vec<char>> = vec![];
    for line in file_to_lines(&path) {
        all_lines.push(line.expect("Could not read liine!").chars().collect());
    }
    all_lines
}

fn is_symbol(char: char) -> bool {
    !char.is_ascii_digit() && char != '.'
}

fn position_has_adjacent_symbol(matrix: &Vec<Vec<char>>, i: usize, j: usize, max_dimensions: &[usize; 2]) -> bool {
    i > 0 && j > 0 && is_symbol(matrix[i - 1][j - 1]) ||
    j > 0 && is_symbol(matrix[i][j - 1]) ||
    i < max_dimensions[0] - 1 && j > 0 && is_symbol(matrix[i + 1][j - 1]) ||
    i > 0 && is_symbol(matrix[i - 1][j]) ||
    i < max_dimensions[0] - 1 && is_symbol(matrix[i + 1][j]) ||
    i > 0 && j < max_dimensions[1] - 1 && is_symbol(matrix[i - 1][j + 1]) ||
    j < max_dimensions[1] - 1 && is_symbol(matrix[i][j + 1]) || 
    i < max_dimensions[0] - 1 && j < max_dimensions[1] - 1 && is_symbol(matrix[i + 1][j + 1])
}

pub fn day_03_first(path: &str) -> u32 {
    let all_lines = construct_character_matrix(path);
    let max_dimensions: [usize; 2] = [all_lines[0].len(), all_lines.len()];
    let mut total: u32 = 0;
    let mut contiguous_symbol: bool;
    let mut current_number = String::default();
    for (i, line) in all_lines.iter().enumerate() {
        contiguous_symbol = false;
        for (j, character) in line.iter().enumerate() {
            if character.is_ascii_digit() {
                current_number.push(character.clone());
                if position_has_adjacent_symbol(&all_lines, i, j, &max_dimensions) {
                    contiguous_symbol = true;
                }
            } else {
                if current_number != "" {
                    if contiguous_symbol {
                        total += current_number.parse::<u32>().expect("Not a number!");
                    }
                    current_number = String::default();
                    contiguous_symbol = false;
                }
            }
        }
        if current_number != "" {
            if contiguous_symbol {
                total += current_number.parse::<u32>().expect("Not a number!");
            }
            current_number = String::default();
        }
    }
    total
}

fn get_left_number(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    let mut number = String::default();
    for pos in (0..j).rev() {
        if matrix[i][pos].is_ascii_digit() {
            number.push(matrix[i][pos]);
        } else {
            break;
        }
    }
    if number.is_empty() {
        None
    } else {
        Some(number.chars().rev().collect::<String>().parse::<u32>().expect("Not a number"))
    }
}

fn get_right_number(matrix: &Vec<Vec<char>>, i: usize, j: usize, max_dimensions: &[usize; 2]) -> Option<u32> {
    let mut number = String::default();
    for pos in j+1..max_dimensions[1] {
        if matrix[i][pos].is_ascii_digit() {
            number.push(matrix[i][pos]);
        } else {
            break;
        }
    }
    if number.is_empty() {
        None
    } else {
        Some(number.parse::<u32>().expect("Not a number"))
    }
}

fn find_numbers_in_row(matrix: &Vec<Vec<char>>, i: usize, j: usize, max_dimensions: &[usize; 2]) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];
    let mut number = String::default();
    if matrix[i][j].is_ascii_digit() {
        number.push(matrix[i][j]);
        for pos in (0..j).rev() {
            if matrix[i][pos].is_ascii_digit() {
                number.push(matrix[i][pos]);
            } else {
                break;
            }
        }
        number = number.chars().rev().collect();
        for pos in j+1..max_dimensions[1] {
            if matrix[i][pos].is_ascii_digit() {
                number.push(matrix[i][pos]);
            } else {
                break;
            }
        }
        out.push(number.parse::<u32>().expect("Not a number"));
    } else {
        for pos in (0..j).rev() {
            if matrix[i][pos].is_ascii_digit() {
                number.push(matrix[i][pos]);
            } else {
                break;
            }
        }
        number = number.chars().rev().collect();
        if !number.is_empty() {
            out.push(number.parse::<u32>().expect("Not a number"));
        }
        number = String::default();
        for pos in j+1..max_dimensions[1] {
            if matrix[i][pos].is_ascii_digit() {
                number.push(matrix[i][pos]);
            } else {
                break;
            }
        }
        if !number.is_empty() {
            out.push(number.parse::<u32>().expect("Not a number"));
        }
    }
    out
}

fn get_upper_or_lower_numbers(matrix: &Vec<Vec<char>>, i: usize, j: usize, max_dimensions: &[usize; 2]) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];
    if i > 0 {
        out = find_numbers_in_row(matrix, i-1, j, max_dimensions);
    }
    if i < max_dimensions[0] {
        out.append(&mut find_numbers_in_row(matrix, i+1, j, max_dimensions));
    }
    out
}

fn get_cog_number(matrix: &Vec<Vec<char>>, i: usize, j: usize, max_dimensions: &[usize; 2]) -> Option<u32> {
    let mut out = get_upper_or_lower_numbers(matrix, i, j, max_dimensions);
    if let Some(n) = get_left_number(matrix, i, j) {
        out.push(n);
    }
    if let Some(n) = get_right_number(matrix, i, j, max_dimensions) {
        out.push(n);
    }
    if let 2 = out.len() {
        Some(out[0] * out[1])
    } else {
        None
    }
}

pub fn day_03_second(path: &str) -> u32 {
    let all_lines = construct_character_matrix(path);
    let max_dimensions: [usize; 2] = [all_lines[0].len(), all_lines.len()];
    let mut total: u32 = 0;
    for (i, line) in all_lines.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            if *character == '*' {
                match get_cog_number(&all_lines, i, j, &max_dimensions) {
                    Some(n) => total += n,
                    None => (),
                }
            }
        }
    }
    total
}
