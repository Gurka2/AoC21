use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


const INPUT: &str = "input";
const TEST: &str = "test";


fn main() {

    let mut inc1: i32 = -1;
    let mut inc2: i32 = -3;

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {

        let mut row1: i32;
        let mut row2: i32 = 0;

        let mut win1 = vec![0; 3];
        let mut win2 = vec![0; 3];

        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() {

                    row1 = row2;
                    row2 = ip.parse().unwrap();
                    inc1 += day_1_part_1(row1, row2);

                    win1.pop();
                    win2.pop();
                    win1.insert(0, row1);
                    win2.insert(0, row2);
                    for value in &win1 {
                        print!("{} ", value);
                    }
                    println!("");

                    inc2 += day_1_part_2(&win1, &win2);
                }
            }
        }
    }

    println!("Result one: {}", inc1);
    println!("Result two: {}", inc2);

}

pub fn day_1_part_1(row1: i32, row2: i32) -> i32 {
    if row2 > row1 {
        return 1;
    }
    return 0;
}

pub fn day_1_part_2(win1: &Vec<i32>, win2: &Vec<i32>) -> i32 {

    let mut sum1 = 0;
    let mut sum2 = 0;

    for value in win1 {
        sum1 += value;
    }

    for value in win2 {
        sum2 += value;
    }

    if sum2 > sum1 {
        return 1;
    }

    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
