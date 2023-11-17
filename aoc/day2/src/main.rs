use std::fs::read_to_string;

const FILENAME: &str = "./input.txt";

fn main() {
    let mut result = 0;
    let (x_score, y_score, z_score) = (1, 2, 3);
    let (loss, draw, win) = (0, 3, 6);

    for line in read_to_string(FILENAME).unwrap().lines() {
        let parsed_line: Vec<&str> = line.trim().split(" ").collect();

        match parsed_line[0] {
            "A" => match parsed_line[1] {
                "X" => result += x_score + draw,
                "Y" => result += y_score + win,
                "Z" => result += z_score + loss,
                _ => {}
            },
            "B" => match parsed_line[1] {
                "X" => result += x_score + loss,
                "Y" => result += y_score + draw,
                "Z" => result += z_score + win,
                _ => {}
            },
            "C" => match parsed_line[1] {
                "X" => result += x_score + win,
                "Y" => result += y_score + loss,
                "Z" => result += z_score + draw,
                _ => {}
            },
            _ => {}
        }
    }

    println!("Result -> {result}");
}
