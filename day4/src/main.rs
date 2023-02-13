use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;


const INPUT: &str = "input";
const TEST: &str = "test";


fn main() {

    let mut is_bingo_time = false;
    let mut numbers: Vec<i32> = Vec::new();

    // Parse the imput file
    if let Ok(lines) = read_lines(TEST) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() && !is_bingo_time {
                    parse_numbers(&mut numbers, &ip);
                } else if ip.is_empty() {
                    is_bingo_time = true;
                }
            }
        }
    }
}

pub fn parse_numbers(numbers: &mut Vec<i32>, numbers_input: &str) {
    for i in 0..numbers_input.len() {
        let value = numbers_input.chars().nth(i).unwrap();
        println!("{}", value);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
