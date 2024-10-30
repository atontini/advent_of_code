use std::fs::File;
use std::io::{self, BufRead};

const PATH: &str = "assets/input.txt";

fn formula(length: i32, width: i32, height: i32) -> i32 {
    let side1 = length * width;
    let side2 = width * height;
    let side3 = height * length;

    let mut smallest_side = 0;
    smallest_side = if side1 < side2 { side1 } else { side2 };
    smallest_side = if smallest_side < side3 { smallest_side } else { side3 };
    
    let mut total = 0;
    
    total += 2 * side1 + 2 * side2 + 2 * side3;
    total += smallest_side;
    return total;
}

fn main() {
    match File::open(PATH) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let mut total = 0;
            for line in reader.lines() {
                match line {
                    Ok(ref content) => {
                        let parts: Vec<&str> = content.split('x').collect();
                        if let [Ok(length), Ok(width), Ok(height)] = [parts[0].parse::<i32>(), parts[1].parse::<i32>(), parts[2].parse::<i32>()] {
                            total += formula(length, width, height);
                        } else {
                            println!("Error: Unable to parse integers from parts");
                        }
                    },
                    Err(e) => eprintln!("Error reading line: {:?}", e),
                }                
            }
            println!("Total feets {}", total);
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}