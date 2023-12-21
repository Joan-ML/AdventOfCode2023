use std::io::{BufRead, BufReader, Lines};
use std::fs::File;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn file_to_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Could not open file!");
    BufReader::new(file).lines()
}
