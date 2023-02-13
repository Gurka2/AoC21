use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;


const INPUT: &str = "input";
const TEST: &str = "test";


fn main() {

    let mut is_bingo_time = false;
    let mut numbers: Vec<i32> = Vec::new();
    let mut bingo_trays: Vec<[[i32; 5]; 5]> = Vec::new();

    // Parse the imput file
    if let Ok(lines) = read_lines(TEST) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() && !is_bingo_time {
                    parse_numbers(&mut numbers, &ip);
                }

                if ip.is_empty() {
                    is_bingo_time = true;
                }

                if is_bingo_time {
                    parse_bingo(&mut bingo_trays, &ip);
                }
            }
        }
    }

    for tray in bingo_trays {
        println!("{:?}", tray);
    }
}

pub fn parse_bingo(bingo_trays: &mut Vec<[[i32; 5]; 5]>, input: &str) {

    // Next row is begining of another bingo tray
    if input.is_empty() {
        bingo_trays.push([[0; 5]; 5]);
    } 

    else {
        for mut tray in bingo_trays[bingo_trays.len()-1] {

            // Non zero means this row is already filled
            for y in tray {
                for x in y {
                    if x != 0 {
                        break;
                    }
                }
            }
        insert_bingo_row(&mut tray, input);
        }
    }
}

pub fn insert_bingo_row(row: &mut [i32; 5], input: &str) {
    let mut x = 0;

    let mut number = String::new();

    for i in 0..input.len() {
        let value = input.chars().nth(i).unwrap();

        // Only chars are 0..9 and ','
        match value {
            ' ' => {
                // If the first value is ' ' skip it
                if i != 0 {
                    row[x] = str_to_i32(&number);
                    number = "".to_string();
                    if input.chars().nth(i+1).unwrap() != ' ' {
                        x += 1;
                    }
                }
            },
            _ => {
                number.push(value);

                // edge case
                if i == input.len()-1 {
                    row[x] = str_to_i32(&number);
                }
            },
        }
    }
}


pub fn parse_numbers(numbers: &mut Vec<i32>, input: &str) {

    let mut number = String::new();

    for i in 0..input.len() {
        let value = input.chars().nth(i).unwrap();

        // Only chars are 0..9 and ','
        match value {
            ',' => {
                numbers.push(str_to_i32(&number));
                number = "".to_string();
            },
            _ => {
                number.push(value);

                // edge case
                if i == input.len()-1 {
                    numbers.push(str_to_i32(&number));
                }
            },
        }
    }
}


pub fn str_to_i32(input: &str) -> i32 {
    println!("input: {}", input);
    if input.is_empty() {
        return 0;
    }

    let mut number = String::new();

    for i in 0..input.len() {
        let value = input.chars().nth(i).unwrap();
        number.push(value);
    }
    return number.parse().unwrap();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
