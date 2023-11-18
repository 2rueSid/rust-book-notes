use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "./input.txt";

fn main() {
    let file = File::open(FILEPATH).expect("File does not exists {FILEPATH}");

    let mut result: u32 = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("Error while reading line");
        let length = line.len();

        let (pt1, pt2) = &line.split_at(length / 2);

        // Create Set from characters
        let pt2_set: HashSet<char> = pt2.chars().collect();
        let pt1_set: HashSet<char> = pt1.chars().collect();

        for item in pt1_set {
            if pt2_set.contains(&item) {
                result += get_pos(item);
                break;
            }
        }
    }

    println!("result -> {}", result);
}

fn get_pos(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}
