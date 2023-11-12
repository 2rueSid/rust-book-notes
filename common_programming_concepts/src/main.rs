fn main() {
    // variables
    // default immutable
    let a: u8 = 2;

    /// mutable
    let mut b: u8 = 10;

    // signed
    let c: i32 = -8;

    // float
    let d: f64 = 9.3;

    const PORT: i32 = 3000;

    let res = a / b;

    print!("{}", res);
    let res = c % 2;
    print!("{}", res);

    let char1: char = 'b';

    let tupl: (i32, f64, char) = (2, 2.22222, 'K');

    println!("{} \n {} \n {}", tupl.0, tupl.1, tupl.2);

    // arrays have fixed length, like in c++, and can't be changed after definition
    let arr: [i32; 5] = [1, 2, 3, 0, 0];
    let arr = [5; 10];
    let arr = [1, 2, 3, 4];

    let gate_to_heaven = 'v';

    no_return_value(22);

    println!("with return value {}", with_return_value(22, 199));
}

fn no_return_value(num: i32) {
    println!("number is {}", num);
}

fn with_return_value(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
