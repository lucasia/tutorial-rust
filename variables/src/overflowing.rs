use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn shadowing() {
    print_method_name("Shadowing");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

pub fn parsing() {
    print_method_name("Parsing");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{}", guess);
}

pub fn wrapping() {
    print_method_name("Wrapping");

    let x: u8 = 255;
    let y: u8 = x.wrapping_add(1);
    println!("Using wrapping_add: {}", y); // Prints "Using wrapping_add: 0"
}

pub fn checked() {
    print_method_name("Checked");

    let x: u8 = 255;
    let y = x.checked_add(1);
    match y {
        Some(value) => println!("Using checked_add: {}", value),
        None => println!("Overflow occurred during checked_add"),
    }
}

pub fn overflowing() {
    print_method_name("Overflowing");

    let x: u8 = 255;
    let (result, overflowed) = x.overflowing_add(1);
    if overflowed {
        println!("Overflow occurred during overflowing_add");
    } else {
        println!("Using overflowing_add: {}", result);
    }
// Prints "Overflow occurred during overflowing_add"
}

pub fn saturating() {
    print_method_name("Saturating");

    let x: u8 = 255;
    let y: u8 = x.saturating_add(1);
    println!("Using saturating_add: {}", y); // Prints "Using saturating_add: 255"
}

pub fn bigint_counter() {
    print_method_name("BigInt Counter");

    let mut a: BigInt = Zero::zero();
    let b: BigInt = One::one();

    for _ in 0..100 {
        a = a + &b; // Notice the reference with &b
    }

    println!("{}", a); // Prints "100"
}

fn print_method_name(name: &str) {
    println!("=============== {} =============== ", name);
}