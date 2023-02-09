use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const INPUT: &str = "input";


fn main() {

    let inc: i32 = 0;

    // Parse the imput file
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {

                if !ip.is_empty() {
                    day_1()



                }
            }
        }
    }
}

pub fn day_1() {

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
