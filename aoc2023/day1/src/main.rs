use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";
const NUMBERS: std::ops::RangeInclusive<char> = '1'..='9';
const NUMBERS_AS_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_number_for_string(str: &str) -> char {
    match str {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}

fn translate_line(line: &String) -> String {
    let mut result = String::new();

    let mut start_position = 0;
    let window_size = 5;

    if line.len() < window_size {
        return line.clone();
    }
    while start_position < line.len() {
        let end_window = std::cmp::min(start_position + window_size, line.len());
        let window = &line[start_position..end_window];
        let mut found = false;

        for number_as_str in &NUMBERS_AS_STR {
            if let Some(pos) = window.find(number_as_str) {
                let end_position = start_position + pos + number_as_str.len();

                if end_position <= line.len() {
                    result.push_str(&line[start_position..start_position + pos]);

                    let number = get_number_for_string(number_as_str);
                    result.push(number);
                    start_position = end_position;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            result.push_str(&line[start_position..start_position + 1]);
            start_position += 1;
        }
    }

    result
}

fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut res: u32 = 0;
    for line in lines {
        let translated = translate_line(&line);

        let mut line_numbers = String::new();

        for c in translated.chars() {
            if NUMBERS.contains(&c) {
                line_numbers.push(c);
            }
        }

        let mut chars = line_numbers.chars();
        let first_number = chars.next().expect("String should be valid here");
        let last_number = chars.last().unwrap_or(first_number);

        let concat = format!("{first_number}{last_number}");

        res += concat.parse::<u32>().expect("Should be a number");
    }

    println!("RESULT -> {res}")
}
