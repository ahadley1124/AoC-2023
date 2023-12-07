use std::fs::File;
use std::io::Read;

fn read_input_file() -> Vec<String> {
    let mut file = File::open("puzzleInput.txt").expect("File not found");
    let mut lines: Vec<String> = Vec::new();
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading the file");
    lines.push(content.split("\n").collect());
    lines
}

fn find_first_number(line: String) -> String {
    let mut first_number = String::new();
    for ch in line.chars() {
        let ascii = ch as u32;
        if ascii >= 48 && ascii <= 57 {
            println!("{}", ascii.to_string());
            let first_number = ascii.to_string();
            return first_number;
            break;
        } else {
            continue;
        }
        break;
    }
    first_number
}

fn find_last_number(line: String) -> String {
    let mut last_number = String::new();
    for ch in line.chars().rev() {
        let ascii = ch as u32;
        if ascii >= 48 && ascii <= 57 {
            println!("{}", ascii.to_string());
            let last_number = ascii.to_string();
            return last_number;
            break;
        } else {
            continue;
        }
        break;
    }
    last_number
}

fn ascii_to_real_number(ascii: u32) -> i32 {
    let mut real_number = 0;
    if ascii >= 48 && ascii <= 57 {
        real_number = ascii as i32;
    }
    real_number
}

fn main() {
    let input = read_input_file();
    let mut sum = 0;
    for line in input {
        let mut first_number: String = find_first_number(line.clone());
        let last_number: String = find_last_number(line.clone());
        first_number.push_str(&last_number);
        let number: i32 = first_number.parse().unwrap();
        sum += number;
    }
    println!("Sum: {}", sum);
}
