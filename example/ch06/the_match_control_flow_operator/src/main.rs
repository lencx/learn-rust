fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    plus_one_example();
    placeholder_example();
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}", state);
            25
        },
    }
}

fn plus_one_example() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}; none: {:?}", six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => {
            println!("i: {:?}", i);
            Some(i + 1)
        },
        None => None,
    }
}

fn placeholder_example() {
    // let some_value = 0u8; // other
    let some_value = 3; // three
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("other"),
    }
}