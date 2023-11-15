enum IpAddrKind {
    Ipv4(u8, u8, u8, u8), // enum values could be of any type structs/tuples/scalars/other enums ...
    Ipv6 { length: i32, value: String },
}

enum Message {
    Quit,                    // Empty Struct
    Move { x: i32, y: i32 }, // Struct
    Write(String),
    ChangeColor(i32, i32, i32), // Tuple Struct
}

impl Message {
    // we can also define methods for enums
    fn call(&self) {
        println!("I was called");
    }
}

fn main() {
    enums_definition();

    let m = Message::Write(String::from("()"));
    m.call();
}

fn enums_definition() {
    let ipv4 = IpAddrKind::Ipv4(1, 0, 0, 127);
    let ipv6 = IpAddrKind::Ipv6 {
        length: 3,
        value: String::from("::1"),
    };
}

fn option_null() {
    // in rust the compiler won't let you do anything with the value when it may be null.
    // for this there is an Option enum that basically has this syntax:
    // enum Option<T> {
    // None,
    // Some(t)
    // }
    // it's so common that it's exists in the global scope
    let a = Some(5);
    let b = 8;

    // now we can't add a + b, because type a is Option<i32> and type b is i32
    // for that we need to parse a to an i32 and if it exists - do the operation
    // everywhere where the value x doesn't have the type Option<t> we can assume that the value is not null. Since rust does'nt have null type, Option is utilized as a null.
}
