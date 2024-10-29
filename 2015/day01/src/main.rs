use std::env;
use std::fs;

const PATH: &str = "assets/input.txt";

fn main() {
    let file_path = PATH;
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut floor = 0;
    let mut position = 1;
    let mut flag = false;

    for c in contents.chars() { 
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }

        if !flag {
            if floor == -1 {
                flag = true;
            }
            else {
                position += 1;
            }
        }
    }

    println!("The first answer is {floor}");
    println!("The second answer is {position}");
}