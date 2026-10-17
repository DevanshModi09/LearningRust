use rand::prelude::*;
use std::{cmp::Ordering, io};

fn main() {
    println!("-----Guess the number !-----");
    let secret_number = rand::rng().random_range(1..=100);
    println!("{secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("An Error Occured");

        println!("You guessed {}", guess);
        let guess: i32 =  match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed right ");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
