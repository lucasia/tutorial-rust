use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!(">> Starting Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secrete number is: {secret_number}"); // debugging only

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // user wants to quit
        match guess.trim().cmp(&"quit".to_string()) {
            Ordering::Equal => {
                println!("Thanks for playing!");
                break;
            }
            _ => {}
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // check guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winna winna chicken dinna!");
                break; // exit loop
            }
        }
    }
}
