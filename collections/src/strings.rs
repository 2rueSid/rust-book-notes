// string in rust is a collection of bytes encoded in utf-8.
// both String and &str (string slice) are strings in rust.
// String in rust implemented as a vector of bytes with some extra methods around.

pub fn exec() {
    str_example();
    str_operations();
    indexing();
    iterating();
}
fn iterating() {
    // we can use chars
    for c in "Зд".chars() {
        println!("{c}");
    }
    // or we can use bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn indexing() {
    // strings can't be accessed by index like in python for example:
    // let s1 = String::from("hello");
    // let h = s1[0]; -> this wont work

    // A String is a wrapper over a Vec<u8>

    let hello = String::from("Hola"); // in this case the length of the vector will be 4
    let hello = String::from("Привіт"); // in this case the length of the vector will be more than 10. It's because how UTF-8 works. because different symbols may be encoded to different number of bytes.

    // So thats why indexing is not possible.

    // A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.
}

fn str_operations() {
    let mut s = String::from("initial contents");

    // we can append string by pushing a string as in vector
    s.push_str("string");

    // or we can push one caracter
    s.push('c');

    // we can also concatenate 2 strings using + operator
    // the reason why we need to pass a reference for the second param
    // is because the function of adding strings look smth like this:
    // fn add(self, s: &str) -> String { -> self is the 1 string
    let mut s2 = String::from("aaaa") + &s;

    // for multiple concatenation we can use format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("s -> {s}");
}

fn str_example() {
    // create new string
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // or use String::from
    let s = String::from("initial contents");

    println!("S is -> {s}");
}
