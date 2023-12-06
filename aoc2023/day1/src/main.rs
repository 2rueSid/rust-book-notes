use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";
fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let numbers = "1234567890";

    let mut res: u32 = 0;
    for line in lines {
        let mut num = String::new();
        for c in line.chars() {
            if numbers.contains(c) {
                num.push(c);
            }
        }

        let mut chars = num.chars();
        let first_char = chars.next().expect("String should not be empty");
        let last_char = chars.last().unwrap_or(first_char);

        let mut concat = String::new();
        concat.push(first_char);
        concat.push(last_char);

        res += concat.parse::<u32>().expect("Should be a number");
    }

    println!("RESULT -> {}", res);
}
