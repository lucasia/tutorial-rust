const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

pub fn shadowing() {

    let x = 5; // x = 5
    println!("x is {x}");

    let x = x + 1; // x' = 6
    println!("x' is {x}");

    {
        let x = x * 2;  // x'' = 12
        println!(">> inner x'' is {x}");

    }

    println!("x' is {x}");  // x' = 6 (still)
}

