extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("perfect!");
                break;
            }
        }
    }
}
