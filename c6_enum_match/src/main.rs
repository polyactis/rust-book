
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    // &Coin instead of Coin because it does not need to be mutated.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        
    }
}


fn add_fancy_hat() {
    println!("Added a fancy hat!");
}
fn remove_fancy_hat() {
    println!("Removed a fancy hat!")
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");

    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    let quarter_coin = Coin::Quarter(UsState::Alaska);

    let quarter_value  = value_in_cents(&quarter_coin);
    println!("quarter_value is {:?}!", quarter_value);
    let quarter_alabama = Coin::Quarter(UsState::Alabama);
    println!("quarter_value is {:?}!", value_in_cents(&quarter_alabama));
    
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}!");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = &quarter_coin {
        //&quarter_coin and quarter_coin are both OK but the latter will move the variable's ownership.
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count is {count}.");

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_char, absent_number);

    
}



