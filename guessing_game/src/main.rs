use rand::prelude::*;
use std::io;

use std::cmp::Ordering;
fn main() {
    println!("Welcome to the Guessing Game , Please Enter A Guess");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too samll , Please Guess Again : "),
            Ordering::Equal => {
                println!("Perfect guess , You win !!!");
                break;
            }
            Ordering::Greater => println!("Your guess is too big , Please Guess Again: "),
        }
    }
}
