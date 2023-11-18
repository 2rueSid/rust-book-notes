use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILEPATH: &str = "./input.txt";

fn main() {
    let file = File::open(FILEPATH).expect("File does not exists {FILEPATH}");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut result: u32;

    result = task_one(&lines);
    println!("result task 1 -> {}", result);

    result = task_two(&lines);
    println!("result task 2 -> {}", result);
}
fn task_two(lines: &Vec<String>) -> u32 {
    let mut res: u32 = 0;
    for chunk in lines.chunks(3) {
        if chunk.len() != 3 {
            continue;
        }

        let vector: Vec<HashSet<char>> = chunk.iter().map(|line| line.chars().collect()).collect();

        for item in &vector[0] {
            if vector[1].contains(&item) && vector[2].contains(&item) {
                res += get_pos(*item);
                break;
            }
        }
    }

    res
}

fn task_one(lines: &Vec<String>) -> u32 {
    let mut result: u32 = 0;
    for line in lines {
        let length = line.len();

        let (pt1, pt2) = &line.split_at(length / 2);

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
