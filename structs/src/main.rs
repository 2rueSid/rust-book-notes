// normal struct
struct User {
    password: String,
    username: String,
    active: bool,
    login_count: u32,
}

// slice struct
struct Square(f32, f32, f32, f32);

// empty struct
struct AlwaysEqual;

fn main() {
    struct_example();

    // structs could also be tuples:
    let square = Square(1.2, 44.2, 1.2, 44.2);

    // structs also can be empty
    let always_equal = AlwaysEqual;
}

fn struct_example() {
    // immutable
    let user = User {
        password: String::from("value"),
        username: String::from("username"),
        active: true,
        login_count: 1,
    };

    // mutable instance
    let mut user = User {
        password: String::from("value"),
        username: String::from("username"),
        active: true,
        login_count: 1,
    };

    user.username = String::from("username22");

    update_instance();
}

fn update_instance() {
    let user1 = user_build(String::from("username"), String::from("password"));

    let user2 = User {
        password: user1.password, // when we perform this operation the user1 instance loses ownership on user1.password field
        username: user1.username, // same as with password, when it loses ownership, we can't access these values of user1 later.
        active: user1.active,
        login_count: 1,
    };
    // error, that indicates that value user1.password was moved
    // println!("{}", user1.password)

    // but we can still use active and login_count fields as they are scalars and implement Copy traits
    println!("{}", user1.active);

    // we can also use destructurization in order to copy values from one struct to another
    let user3 = User {
        active: false,
        ..user2
    };
}

fn user_build(username: String, password: String) -> User {
    User {
        active: true,
        login_count: 1,
        password, // if the name of the parameter is the same as the key of the field in struct we can just pass this parameter.
        username, // sames as with password
    }
}
