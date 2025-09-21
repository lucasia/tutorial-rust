use log::debug;

pub fn control_flow() {
    if_else();
    if_statement();
    loops();
    whiles();
    fors();
}

fn fors() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        debug!("the value is: {element}");
    }

    for number in (1..4).rev() {
        debug!("{}!", number);
    }
    debug!("LIFTOFF!!!");
}

fn whiles() {
    let mut number = 3;
    while number != 0 {
        debug!("{number}!");

        number -= 1;
    }

    debug!("LIFTOFF!!!");
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    debug!("The result is {}", result);
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        debug!("number is divisible by 4");
    } else if number % 3 == 0 {
        debug!("number is divisible by 3");
    } else if number % 2 == 0 {
        debug!("number is divisible by 2");
    } else {
        debug!("number is not divisible by 4, 3, or 2");
    }
}

fn if_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    debug!("The value of number is: {number}");
}
