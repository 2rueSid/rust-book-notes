## Format

path can take 2 forms.

- **absolute path** - is the full path that starts with `crate::`
- **relative path** - starts from the current module and used `self`, `super` or an identifier in the current module.

## `pub` Keyword

This code wont run because the content of the module and each expression is `private` by default.

```rust
// in order to compile this we will need to include pub keyword
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

So we need to include `pub` keyword for each element starting from the parent module.

```rust
pub mod front_of_house { // now module is pub, but content stil private
....
}
```

```rust
// now it will compile
pub mod front_of_house {
   pub mod hosting {
       pub fn add_to_waitlist() {}
    }
}
```

## `super` or access parent content

To access parent content me can use `super` keyword it's like `..` in unix system.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Struct and Enum

When we define `struct` or `enum`, we also may utilize `pub` keyword.

In case of struct this will only make struct accessible, the content will be private.
Making content public should be done case by case basis.

When the struct have private field, we need to provide a public associated function, that construct an instance of struct.
If the private value don't have a constructor, we can't create a struct instance, because we can't specify value for a private field.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // this will be private
    }

    impl Breakfast {
        // contructor for private value
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

In contrast for enums, when we make the enum public, all it's content will be also public.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## More about `use` keyword

When using `use` keyword there are semantic rules.

when we need to get functions or variables, it's better to use path to the module instead of the direct definition:

```rust
pub mod some {
  pub mod bb {
    pub fn a() {}

    pub struct Bc
  }
}

  // for functions
  use crate::some::bb

  // for structs, enums
  use crate::some::bb::Bc

fn test() {
  bb::a();

  let c = Bc;
}
```

if we get a name conflict, eg 2 modules have same name for a struct we can specify local name using `as` keyword.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Reexporting

we can **reexport** name by using `pub use`, this will allow for calling module to refer to that name if that it have been defined in that scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## Using Nested Paths to Clean Up Large use Lists
we can import many modules using one statement to clean imports part.

from this:
```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

to this:
```rust
use std::{cmp::Ordering, io};
```

or 
```rust
use std::io;
use std::io::Write;
```
in this case the same part is `io` module, so we can refer to it as a `self`.
```rust
use std::io::{self, Write};
```

of we can import all public methods of the module to the scope by using glob operator.

```rust
use std::collections::*;
```