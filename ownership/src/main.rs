fn main() {
    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    takes_ownership(s);

    // println!("s is invalidated, and it's not longer exists {s}");

    let b = 3;
    makes_copy(b);

    println!("b exists {b}");

    return_values();
}

// returning values can also transfer ownership
fn return_values() {
    fn give_ownership() -> String {
        let s = String::from("some string");

        s
    } // s is returned and transfers ownership to the calling function

    fn takes_and_gives_back(s: String) -> String {
        println!("Took string {s}");

        s
    }

    let s1 = give_ownership();

    let s2 = String::from("value");
    // s2 is invalidated here and ownership is transferred to s3
    let s3 = takes_and_gives_back(s2);

    // println!("s2 is invalidated {s2}");
}

// passing parameter to a function works pretty much like a variable assignment.
// for scalar values eg integer it will copy the variable and creates a new variable in the stack.
// such a: parameter x1 and a variable that we pass it's x the stack will be |x1, x|
// and when the function is out of scope the stack will remove x1. but x will exist in the place from where this function was called.
// for non-scalar types, the behavior is next: when function is called in and we pass parameter such as x1 that is of type String, we also give ownership to this parameter, so when the function is out of scope, the x1 is deleted from heap and stack, both in function that was called and in function from which it was called.
fn takes_ownership(s: String) {
    // the parameter will take ownership
    println!("String is {s}");
} // the s will be invalided here as well as the the variable in place from were it was called

fn makes_copy(i: i32) {
    // here the i will copy the value from the variable that is passed as a param
    println!("Integer is {i}");
} // i will be invalidated but the variable that was passed as a param will exists.

// This called a MOVE. like S1 was MOVED into S2
fn dynamicDTWnershipExample() {
    // rule #1 value in heap cna have only 1 owner.
    let s1 = String::from("hello");

    // try to copy the value of s1 to s2
    let s2 = s1;

    // this example show that we are trying to copy the s1 and assign it to s2.
    // in python for example we would have a variable of s2 that if it's changed the s1 will also be changed.
    // in rust on the other hand the variable s1 is marked as OUT OF SCOPE, means that it dropped from the stack and the heap, and ownership transferred to s2. or simply s1 becomes invalid

    // this will be a compiler error, since the s1 is out of scope now.
    // println!("{}", s1);

    // this is the right variant
    print!("{}", s2);

    // this is done on order to prevent the situations where we have 2 variables that are pointing
    // to the same location in heap, and when they goes out of scope compiler would try to
    // free already non existing memory, or they both will try to free the same memory.

    // to really copy the variable we can run clone method
    let s3 = s2.clone();
    println!("{}, {}", s2, s3);

    // but it only actually valid if the value is stored in the heap, or it dont have a dynamic size allocation.
    // types like integer, float, char, array, slice, boolean are stored in the stack, as we know their size on creation.
    let x = 1;
    let x1 = x;

    println!("{}, {}", x, x1);
}
