fn main() {
    println!("Hello, world!");
    match_value();
    if_let_value();

    coin_match(Coin::Penny);
    coin_match(Coin::Quarter(UsState::Alabama));

    coin_if_let(Coin::Quarter(UsState::Alabama));
}

fn match_value() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("match => three"),
        _ => (),
    }
}

fn if_let_value() {
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("if let => three");
    }
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

fn coin_match(coin: Coin) {
    println!("********* coin_match **********");
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn coin_if_let(coin: Coin) {
    let mut count = 0;
    println!("********* coin_if_let **********");
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}