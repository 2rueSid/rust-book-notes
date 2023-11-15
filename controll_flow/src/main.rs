#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// we can also evaluate Option<T> expressions using match
fn mult(x: Option<i32>, y: i32) -> Option<i32> {
    match x {
        None => None,           // if x is None -> return None
        Some(i) => Some(i * y), // Else Multiply
    }
}

fn value_in_coin(v: Coins) -> u8 {
    // match is similar to if, but if evaluates only boolean, and match can evaluate any value.
    // when match is executed it compares first part of the expression called pattern of arm one by one.
    // Match should cover all possible variants. If it for example does not cover Penny -> compiler will throw error, that tells that pattern Penny is not covered.
    match v {
        Coins::Penny => 1,  // arm consist of pattern and expression
        Coins::Nickel => 5, // expression can be any expression like {} for example
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            // we can also set parameters, for binded values
            println!("State quarter from {:?}!", state);

            25
        }
    }
}

fn default_case() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other_val => move_player(other_val), // match can also have default case. here other_val is a variable that covers all other possible cases.
                                             // _ => do_some() - if pattern dont need to have variable
                                             // _ => () - in this case we tell compiler that we wont expect other values than 3 and 7
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// if let syntax allows to shorter control flow, like for example if we need to cover only one case for non boolean value
fn if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // we don't need to handle None, so we just skip it
    }

    // if let allows us to write a shorter and cleaner handling
    // if let takes pattern and expression separated by = sign.
    // in this case the pattern is Some(max) and max binds to the value inside Some. This allows using max in the expression.
    // In other words, if let used to run code  when the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Default.");
    } // we can also use else expression here
}

fn main() {
    let penny = value_in_coin(Coins::Penny);

    println!("penny\t{penny}");

    let quarter = value_in_coin(Coins::Quarter(UsState::Alabama));

    println!("quarter\n{quarter}");

    let x = Some(228);
    let res = mult(x, 10);
    let x1: Option<i32> = None;
    let res2 = mult(x1, 22);
}
