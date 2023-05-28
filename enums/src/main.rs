enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum MyMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl MyMessage {
    fn call(&self) {
        // method body would be defined here
        println!("You rang?!?")
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // snip
}

fn main() {
    ip_example();
    message_example();
    option_example();

    value_in_cents(Coin::Quarter(UsState::Alaska));

    option_match_example();
    option_match_advanced_example();

    if_let_example();
}

fn if_let_example() {

    let config_max = Some(3u8);

    // match method
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // if let method
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Thanks for Nothing!");
    }

}

fn option_match_advanced_example() {
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Winner!"),
        7 => println!("Loser!"),
        _ => ()
    }
}


fn option_match_example() {
    let five = Some(5);
    let twenty_five = square(five);
    let none = square(None);

    println!("{:?}, {:?}", twenty_five, none);
}

fn square(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * i),
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
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn option_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{}, {}, {}", some_number.is_some(), some_string.is_some(), absent_number.is_none())
}

fn message_example() {
    let m = MyMessage::Write(String::from("hello"));
    m.call();
}

fn ip_example() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("00000000000"));

    route(four);
    route(six);
}

fn route(ip: IpAddrKind) {
    // do something...
}
