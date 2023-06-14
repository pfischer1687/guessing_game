use rand::prelude::*;
use std::io;

fn main() {
    let rand_num: i32 = thread_rng().gen_range(1..101);
    let mut buffer: String = String::new();
    let guess: i32;
    
    println!("I have generated a random number between 1 and 100. Please keep entering your guesses until you get the correct answer.");
    println!("Enter your guess:");
    _ = io::stdin().read_line(&mut buffer);
    guess = buffer.trim().parse().unwrap();

    println!("rand_num: {rand_num}, guess: {guess}");
}
