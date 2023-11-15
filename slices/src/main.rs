// slices let's us reference the part of the sequence of elements rather that whole value
// a slice is kind of a reference so it does not have ownership

fn main() {
    string_slice();
    string_literals();

    // arrays can also be sliced
    let a = [1, 2, 3, 4, 5, 6, 7];

    let slice = &a[..5];
}

fn string_slice() {
    // string slice it's a reference to a part of the string
    let s = String::from("hello, world!");

    // slices are written in the brackets [starting_index..ending_index]. we can skip 0 if we want to start from zero [..ending_index], or we can skip last index if we need a string from some index like [starting_point..] or we can reference to the whole string like [..]

    // it could also be [0..2]
    let r1 = &s[..5]; // hello
                      // it could also
    let r2 = &s[7..]; // world!

    println!("{r1} --- {r2}");

    print!("first word: {}", first_word(&s));

    let mut s1 = String::from("string 1");

    let word = first_word(&s1); // now word is immutable reference to s1

    // so this behavior will cause an error because the word is immutable reference to s1, so it can't be changed.
    // s1.clear(); // this is the problematic code
    println!("the first word is: {}", word);
}

fn string_literals() {
    // string literals are immutable slices:
    let s = "Hello"; // type is &str, and &str is an immutable reference

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
