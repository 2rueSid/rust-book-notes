use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const FILEPATH: &str = "./input.txt";

fn main() {
    let file = File::open(FILEPATH).expect("File does not exists {FILEPATH}");

    let lower_case = 'a'..='z';
    let upper_case = 'A'..='Z';

    let mut result: u32 = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("Error while reading line");
        let length = line.len();

        let splitted_line = &line.split_at((length / 2));

        for item in splitted_line.0.chars().into_iter() {
            if splitted_line.1.contains(item) {
                if lower_case.contains(&item) {
                    result += get_pos(&lower_case, item) + 1;
                } else {
                    result += get_pos(&upper_case, item) + 27;
                }

                break;
            }
        }
    }

    println!("result -> {}", result);
}

fn get_pos(range: &RangeInclusive<char>, item: char) -> u32 {
    range
        .clone()
        .position(|x| x == item)
        .expect("Error while getting index") as u32
}
