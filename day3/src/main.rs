use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;


const INPUT: &str = "input";
const TEST: &str = "test";
const TOTAL: i32 = 4095;


fn main() {

    let mut rate: Vec<i128> = vec![0; 13];
    let mut values = Vec::new();

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() {
                    let code: String = ip.parse().unwrap();

                    part1(&code, &mut rate);                    
                    values.push(code.to_string());
                }
            }
        }
    }

    println!("Part 1: {}", calc_gamma_eps(&rate) * (TOTAL-calc_gamma_eps(&rate)));
    println!("Part 2: {}", get_life_rating(&values, 0, true) * get_life_rating(&values, 0, false));
    println!("{}", bi_to_dec("0000000000000"));
}

pub fn get_life_rating(values: &Vec<String>, index: usize, oxy: bool) -> i128 {

    if values.len() == 1 {
        return bi_to_dec(&values[0]);
    }

    let mut i: usize = index;

    let mut zero = Vec::new();
    let mut one = Vec::new();

    for value in values {
        value.chars().nth(i);
        if value.chars().nth(i).unwrap() == '0' {
            zero.push(value.to_string());
        } else {
            one.push(value.to_string());
        }
    }

    i += 1;

    if oxy {
        if one.len() == 1 && 1 == zero.len() {
            return bi_to_dec(&one[0]);
        } else if one.len() == zero.len() {
            return get_life_rating(&one, i, oxy);
        } else {
            if one.len() > zero.len() {
                return get_life_rating(&one, i, oxy);
            } else {
                return get_life_rating(&zero, i, oxy);
            }
        }
    } else {
        if one.len() == 1 && 1 == zero.len() {
            return bi_to_dec(&zero[0]);
        } else if one.len() == zero.len() {
            return get_life_rating(&zero, i, oxy);
        } else {
            if one.len() > zero.len() {
                return get_life_rating(&zero, i, oxy);
            } else {
                return get_life_rating(&one, i, oxy);
            }
        }
    }
}

pub fn part1(code: &String, rate: &mut [i128]) {
    for (i, bit) in code.chars().enumerate() {
        rate[i] += bit as i128 - 48;
    }
    rate[12] += 1;
}

pub fn bi_to_dec(code: &str) -> i128 {
    let len = code.chars().count() as u32;
    let mut dec: i128 = 0;

    for (i, value) in code.chars().enumerate() {


        dec += (value as i128 - 48)*2_i128
            .pow((len - i as u32 - 1)
                 .try_into()
                 .unwrap());
    }
    return dec;

}

pub fn calc_gamma_eps(rate: &[i128]) -> i32 {

    let threshold = rate[12]/2;
    let mut pos: i32 = 0;
    let mut gamma: i32 = 0;

    for value in rate {
        if value >= &threshold {

            gamma += 2_i32.pow((11-pos).try_into().unwrap());
        }


        if pos == 11 {
            break;
        }
        pos += 1;
    }

    return gamma;

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
