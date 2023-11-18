use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "./input.txt";

fn main() {
    let file = File::open(FILEPATH).expect("File does not exists {FILEPATH}");

    let result: u32;

    // result = task_one(&file);
    // println!("result task 1 -> {}", result);

    result = task_two(&file);
    println!("result task 2 -> {}", result);
}
fn task_two(file: &File) -> u32 {
    let mut res: u32 = 0;
    let mut vector: Vec<HashSet<char>> = vec![];
    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Error while reading line");
        let lien_set: HashSet<char> = line.chars().collect();
        vector.push(lien_set);

        if vector.len() != 3 {
            continue;
        }

        for item in &vector[0] {
            if vector[1].contains(&item) && vector[2].contains(&item) {
                res += get_pos(*item);
                vector.clear();
                break;
            }
        }
    }

    res
}

fn task_one(file: &File) -> u32 {
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

    result
}

// https://internals.rust-lang.org/t/iterating-over-range-char/8965
fn get_pos(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}
