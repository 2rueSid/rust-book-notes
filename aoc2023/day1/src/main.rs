mod v2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";

fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let res = v2::v2::calculate(&lines);

    println!("RESULT -> {res}")
}
