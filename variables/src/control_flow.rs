use crate::functions::five;

pub fn if_fn() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub fn loop_fn() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter >= 5 {
            break counter * 2;
        }

        println!("again!");
    };

    println!("The result is {result}");
}

pub fn loopedy_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn countdown_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn countdown_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


pub fn for_fn() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}