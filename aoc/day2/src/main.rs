use std::fs::read_to_string;

const FILENAME: &str = "./input.txt";

fn main() {
    let mut result = 0;
    let (rock, paper, scissors) = (1, 2, 3);
    let (loss, draw, win) = (0, 3, 6);

    for line in read_to_string(FILENAME).unwrap().lines() {
        let parsed_line: Vec<&str> = line.trim().split(" ").collect();

        match parsed_line[0] {
            "A" => match parsed_line[1] {
                "X" => result += scissors + loss,
                "Y" => result += rock + draw,
                "Z" => result += paper + win,
                _ => {}
            },
            "B" => match parsed_line[1] {
                "X" => result += rock + loss,
                "Y" => result += paper + draw,
                "Z" => result += scissors + win,
                _ => {}
            },
            "C" => match parsed_line[1] {
                "X" => result += paper + loss,
                "Y" => result += scissors + draw,
                "Z" => result += rock + win,
                _ => {}
            },
            _ => {}
        }
    }

    println!("Result -> {result}");
}
