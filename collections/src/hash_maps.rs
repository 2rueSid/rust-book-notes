use std::collections::HashMap;

pub fn exec() {
    create();
}

fn create() {
    let mut mp = HashMap::new();
    mp.insert(String::from("value1"), 32);
    mp.insert(String::from("value2"), 64);

    // access by key name
    let key = String::from("value1");
    let v: i32 = mp.get(&key).copied().unwrap_or(0);

    // iterate over hash map
    for (key, value) in &mp {
        println!("{key}: {value}");
    }

    // when we pass the types that don't implement Copy trait, the rules of ownership applied
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // at this point field_name and field_value are invalid
}
