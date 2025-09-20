use log::debug;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn mutability() {
    let mut x = 5;
    debug!("The value of x is: {x}");

    x = 6;
    debug!("The value of x is: {x}");

    debug!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

pub fn shadowing() {

    let x = 5; // x = 5
    debug!("x is {x}");

    let x = x + 1; // x' = 6
    debug!("x' is {x}");

    {
        let x = x * 2;  // x'' = 12
        debug!(">> inner x'' is {x}");

    }

    debug!("x' is {x}");  // x' = 6 (still)
}