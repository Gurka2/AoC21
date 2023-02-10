use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const INPUT: &str = "input";
const TEST: &str = "test";


fn main() {

    let mut pos1: [i128; 2] = [0; 2];
    let mut pos2: [i128; 3] = [0; 3];

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() {
                    let command: String = ip.parse().unwrap();
                    day_1_part_1(&command, &mut pos1);
                    day_1_part_2(&command, &mut pos2);
                }
            }
        }
    }

    let p1 = pos1[0]*pos1[1];
    println!("Result part1: {}", p1);
    let p2 = pos2[0]*pos2[2];
    println!("Result part2: {}", p2);

}

pub fn day_1_part_1(command: &String, pos: &mut[i128]) {

    let mut nr_index = 0;
    for (i, item) in command.chars().enumerate() {
        if item == ' ' {
            nr_index = i + 1;
        }
    }

    let number = command.chars().nth(nr_index).unwrap().to_digit(10).unwrap();
    let number2 = number as i128;

    match nr_index {
        8 => pos[0] += number2,
        5 => pos[1] += number2,
        3 => pos[1] -= number2,
        _ => println!("Something is wrong"),

    }
}

pub fn day_1_part_2(command: &String, pos: &mut[i128]) {

    let mut nr_index = 0;
    for (i, item) in command.chars().enumerate() {
        if item == ' ' {
            nr_index = i + 1;
        }
    }

    let number = command.chars().nth(nr_index).unwrap().to_digit(10).unwrap();
    let number2 = number as i128;

    match nr_index {
        8 => {
            pos[0] += number2;
            pos[2] += number2*pos[1];
        },
        5 => pos[1] += number2,
        3 => pos[1] -= number2,
        _ => println!("Something is wrong"),

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
