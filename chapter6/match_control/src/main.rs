fn main() {

}

// Section: catch-all patterns and the _ Placeholder
fn catch_all_with_no_code_to_run() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),  // unit value
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

fn using_a_placeholder_to_catch_remaining_variants() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),  // catch-all pattern when value binding is not needed
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}


fn using_a_variable_to_catch_remaining_variants() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  // catch-all pattern must always be the last one
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// Section: Matching with Option<T>
fn using_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}

// Section: Patterns that bind to values
fn testing_enum_matching() {
    let coin = SpecialCoin::Quarter(UsState::Oregon);
    coin.value_in_cents();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Oregon,
}

enum SpecialCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl SpecialCoin {
    fn value_in_cents(&self) -> u8 {
        match self {
            SpecialCoin::Penny => 1,
            SpecialCoin::Nickel => 5,
            SpecialCoin::Dime => 10,
            SpecialCoin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            } 
        }
    } 
}



// Section: implementing methods in an Enum

fn calling_a_method_from_a_enum() {
    let coin = Coin::Nickel;
    println!(
        "Value in cents: {}",
        coin.value_in_cents()
    )
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
