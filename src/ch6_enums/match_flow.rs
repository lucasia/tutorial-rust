use log::debug;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    // you get the idea...
}

pub fn match_flow() {
    for c in [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
    ] {
        value_in_cents(c);
    }

    let dice_roll = 9;
    match dice_roll {
        7 => debug!("lucky 7!"),
        other => debug!("{}", other), // we could match on "_" instead
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            debug!("State quarter from {state:?}!");
            25
        }
    }
}
