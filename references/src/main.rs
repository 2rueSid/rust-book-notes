// in any point of time you can have multiple immutable references or only one mutable reference
// references must always be valid and reference to existing value
fn main() {
    reference();
    some_restrictions();
}

fn some_restrictions() {
    // we also can't have combination of mutable and immutable references.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // but this scenario will work since all immutable references are becoming out of scope since they last used
    println!("{r1} - {r2}");

    // here the references r1 and r2 are already out of scope since, they last usage was in the print above.
    let r3 = &mut s; // OK, since r1 and r2 are dropped
    println!("{r3}");


}

fn reference() {
    // this example shows reference in action.
    // reference are immutable by default
    let s1 = String::from("hello");
    let len = get_len(&s1);
    println!("{len} : {s1}");

    // to create a mutable reference we need to create a mutable variable first
    let mut s2 = String::from("bla bla");
    let len = mutable_get_len(&mut s2); // we pass a mutable reference

    println!("Mutable ref {len} - {s2}");

    // but mutable references has one big restriction:
    // we can't have other references to that value.
    // let r1 = &mut s2; // 1-rst borrowing
    // this will throw an error as we can't borrow s2 value as mutable more than once;
    // let r2 = &mut s2; // second borrowing

    // println!("{r1}"); // and compiler will prevent from using these references.

    // this restriction is here to deal with the data race which may occur when:
    // two or more pointers references the same data at the same time
    // at least one pointer was used to write the data
    // there is no mechanism being used to synchronize the data

    // but still we can utilize scope mechanism like this, to have multiple mutable references:
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// passing reference is a process also called borrowing, when we pass a reference instead of actual value to save the ownership for the original variable.
// if we have function get_len that accepts string and returns length we can pass a reference of the string in order to not give the full ownership to that value, instead just reference to it.

fn get_len(s: &String) -> usize {
    // s: &String - pass a string reference to parameter s
    s.len() // return the length of the string
} // s is out of scope and it's value is removed. but since it was a reference to point in memory, the original value is still valid.

fn mutable_get_len(s: &mut String) -> usize {
    // we pass a mutable reference
    s.push_str(" new v"); // we mutate original value
    s.len() // and calculate length
} // reference variable is out of scope and is dropped

