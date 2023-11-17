// vector is a data structure that sequentially stores elements of one type in memory aka heap.
pub fn exec() {
    create_vector();
    iter();
    enums_example();
}

fn create_vector() {
    // Create an empty vector specifying the type. This is necessary if the vector is not immediately initialized with values.
    let v: Vec<i32> = Vec::new();

    // Alternatively, initialize a vector with a macro. Make it mutable if elements will be added later.
    let mut v = vec![1, 2, 3];

    // Add elements to the end of the vector using `push`.
    v.push(-1);

    // Access vector elements by index or using the `get` method.
    v[1]; // Returns the value or panics if the index is out of bounds.
    v.get(1); // Returns an Option<T>, allowing safe handling of out-of-bounds access.

    // Note: Standard ownership and borrowing rules apply to vectors.
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // Error: mutable borrow occurs here. Immutable borrow already taken.
    // v.push(6);

    println!("The first element is: {}", first);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enums_example() {
    // Vectors can store any single type, including enums, allowing for multiple data types in one vector.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn iter() {
    let v = vec![100, 32, 57];
    // Iterate over the vector elements.
    for i in &v {
        println!("{}", i);
    }

    // Mutate elements during iteration.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
