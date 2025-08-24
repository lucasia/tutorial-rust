use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!(">> Starting Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secret_number}");

    println!("Enter your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("Winna winna chicken dinna!"),
    //
    // }

    println!("You guessed: {guess}");
}
