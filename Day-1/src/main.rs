use std::fs::File;
use std::io::Read;

fn read_input_file() -> Vec<String> {
    let filename = "puzzleInput.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
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
    match ascii {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        _ => panic!("Not a number"),        
    }
}

fn main() {
    //let input = parse_input_file(read_input_file());
    let input = read_input_file();
    let mut sum = 0;
    for line in input {
        let mut first_number: String = find_first_number(line.clone());
        let last_number: String = find_last_number(line.clone());
        let mut first_number: u32 = first_number.parse().unwrap();
        let last_number: u32 = last_number.parse().unwrap();
        let first_digit = ascii_to_real_number(first_number);
        let last_digit = ascii_to_real_number(last_number);
        // append the last digit to the first digit
        let numbers: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        sum += numbers;
    }
    println!("Sum: {}", sum);
}
