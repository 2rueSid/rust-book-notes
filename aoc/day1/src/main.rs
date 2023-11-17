use std::fs::read_to_string;

fn main() {
    let mut vec: Vec<u32> = Vec::new();

    let filename: &str = "./input.txt";
    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        if line.trim().as_bytes() == b"" {
            vec.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }

    print_max(&vec);
    print_sum_of_top_3(&vec);
}

fn print_max(vec: &Vec<u32>) {
    let res = vec.iter().max();

    if let Some(max) = res {
        println!("Max value is -> {max}")
    }
}

fn print_sum_of_top_3(vec: &Vec<u32>) {
    let mut new_vec = vec.clone();
    new_vec.sort_by(|a, b| b.cmp(a));

    let top_3 = &new_vec[0..3];

    let mut top3_sum = 0;
    for elem in top_3.iter() {
        top3_sum += elem;
    }
    println!("sum of top 3 is: {top3_sum}");
}
