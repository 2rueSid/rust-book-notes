// A crate is the basic compilation unit in Rust. It's essentially a package of Rust code. When we compile a Rust program, we are compiling a crate.
// A crate can be either a binary crate (which generates an executable) or a library crate (which generates a library that other crates can link against).
// Crates can contain modules, functions, structs, traits, enums, and other items, all this will be compiled in a single unit.

// in cargo the  src/main.rs is a root of a binary crate. also called crate root.
// otherwise if src contains lib.rs - it's a root for the library crate

// the modules cheat sheet:
// compiler start from the crate root.
// in root we can declare modules using mod keyword.
// for example if we declare mod jsonpath, the compiler will look in the src/jsonpath.rs or src/jsonpath/mod.rs
// In any file other than crate root, we can declare sub modules. for example we will declare mod parser in src/jsonpath.rs, so the compiler will look in:
// src/jsonpath/parser.rs or src/jsonpath/parser/mod.rs
// once the module is defined within crate, we can refer to code in that module from anywhere else from this crate by using the path to the code;
// for example JsonParser struct in src/jsonpath/parse/mod.rs
// crate::jsonpath::parser::JsonParser
// code within modules is private by default for their parent modules. To make a module public we can define it with pub keyword like: pub mod instead of mod.
// then we able to make code within module public aswell by defining expressions with pub keyword
// the use keyword within the scope creates a shortcut to items.
// use crate::jsonpath::parser::JsonParser will create shortcut to JsonParser, so within scope we will be able to use just JsonParser.

// create a shortcut for JsonParser
use jsonpath::parser::JsonParser;

// include module jsonpath into the crate
pub mod jsonpath;

// simply return JsonParser
fn get_parser() -> JsonParser {
    JsonParser
}

fn main() {
    let parser = get_parser();
}
