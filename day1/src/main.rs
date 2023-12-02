use std::fs::File;
use std::io::prelude::*;


fn main() {
    let contents = read_file("./input");
    let sum = parse_input(&contents);
    println!("Sum: {}", sum);
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

fn parse_input(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let digits: String = line.chars().filter(|c| c.is_digit(10)).collect();

        if let (Some(first), Some(last)) = (digits.chars().next(), digits.chars().last()) {
            let two_digit_num = format!("{}{}", first, last).parse::<u32>().unwrap();
            sum += two_digit_num;
        }
    }

    sum
}

