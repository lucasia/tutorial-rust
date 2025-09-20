use log::debug;

pub fn ownership() {
    move_example_one();
    move_example_two();
    clone_example();

    function_ownership_example();
    return_values_example();
}


fn move_example_one() {
    let x = 5;
    let y = x;
    debug!("x: {}, y: {}", x, y);


    let s1 = String::from("hello");
    let s2 = s1;    // s1 moved, no longer able to be used!

    debug!("{s2}, world!");
}

fn move_example_two() {
    let mut s = String::from("hello");
    debug!("{s}");
    s = String::from("ahoy");

    debug!("M2: {s}, world!");
}

fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    debug!("s1={s1}, s2={s2}");
}

// ======================
// function ownership
// ======================
fn function_ownership_example() {
    let s = String::from("hello");
    takes_ownership(s); // s moved to function
    // s no longer available to us!
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    debug!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    debug!("{some_integer}");
}

// ======================
// return values and scope
// ======================

fn return_values_example() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 is moved here

    // s2 no longer available to us
    
    debug!("s1={s1}, s3={s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned moves out to the calling function
}