use std::fs::File;
use std::io::{self, BufRead};

const FILENAME: &str = "./input.txt";

enum Variant {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn main() -> io::Result<()> {
    let mut result = 0;

    let file = File::open(FILENAME)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parsed_line: Vec<&str> = line.split_whitespace().collect();

        let varian = match parsed_line[0] {
            "A" => Variant::Scissors,
            "B" => Variant::Rock,
            "C" => Variant::Paper,
            _ => continue,
        };

        let outcome = match parsed_line[1] {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => continue,
        };

        let var_result = match varian {
            Variant::Rock => 1,
            Variant::Paper => 2,
            Variant::Scissors => 3,
        };

        result += var_result + outcome as i32;
    }

    println!("Result -> {result}");
    Ok(())
}
