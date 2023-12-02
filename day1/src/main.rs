use std::fs::File;
use std::io::prelude::*;


fn main() {
    let contents = read_file("./input");
    // let contents = read_file("./sample_input");
    let sum = parse_input(&contents);
    println!("Sum: {}", sum);

    let sum = parse_input_words(&contents);
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


// Some lines have words for the digits:
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen

fn parse_input_words(input: &str) -> u32 {
    let mut sum = 0;

    let words = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for line in input.lines() {
        println!("orig: {}", line);
        println!("len: {}", line.len());
        let mut digit_vec: Vec<Option<String>> = vec![None; line.len()];
        println!("vec_cap: {}", digit_vec.len());
        for &(word, replacement) in words.iter() {
            // At the index where the work is found insert the replacement
            let word_indexes = find_word_indexes(line, word);
            for idx in word_indexes {
                println!(">>word: {}", word);
                println!(">>insert_{}_at: {}", replacement, idx);
                digit_vec[idx] = Some(replacement.to_string());
            }
        }
        for (idx, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                println!(">>insert_{}_at: {}", c, idx);
                digit_vec[idx] = Some(c.to_string());
            }
        }


        println!("vec: {}", digit_vec.len());
        let line_cp = digit_vec.into_iter().filter_map(|x| x).collect::<String>();
        println!("new: {}", line_cp);

        let digits: String = line_cp.chars().filter(|c| c.is_digit(10)).collect();
        print!("{}\n", digits);

        if let (Some(first), Some(last)) = (digits.chars().next(), digits.chars().last()) {
            let two_digit_num = format!("{}{}", first, last).parse::<u32>().unwrap();
            sum += two_digit_num;
            print!("{}\n", two_digit_num);
        }
        println!("-------------------------");
    }

    sum
}

fn find_word_indexes(input: &str, word: &str) -> Vec<usize> {
    let mut indexes = Vec::new();
    let mut start = 0;
    while let Some(idx) = input[start..].find(word) {
        start += idx;
        indexes.push(start);
        start += 1;
    }
    indexes
}
