#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Tuple struct
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/// Matches are exhaustive. Rust knows that which pattern we forgot.
fn match_with_option_T(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_all_pattern() {
    let dice_roll = 9;
    match dice_roll {
        5 => is_five_plus_one(),
        6 => is_six_minus_two(),
        7 => (),
        other => other_number_is_zero(other),
    }
}

fn is_five_plus_one() {
    println!("6");
}
fn is_six_minus_two() {
    println!("4");
}
fn other_number_is_zero(num: i32) {
    println!("{}", num);
}

/// You lose the exhaustive checking that match enforces when you use `if let`. In other words, you can think of `if let` as syntax sugar for a `match`.
fn if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

/// Use `else` to match all pattern when you use `if let`
fn if_let_match_all_pattern() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn main() {
    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Texas)));
    let five = Some(5);
    let six = match_with_option_T(five);
    let none = match_with_option_T(None);
    match_all_pattern();
    if_let();
    if_let_match_all_pattern();
}
