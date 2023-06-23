use rand::prelude::*;
use std::io;
use std::num::ParseIntError;

fn main() {
    
    let mut stdin_result: Result<usize, io::Error>;
    let mut _stdin: usize;
    let mut guess_result: Result<i32, ParseIntError>;
    let mut guess: i32;
    let num_iter: i32 = 5;
    let rand_num: i32 = thread_rng().gen_range(1..101);
    
    println!("I have generated a random number between 1 and 100. Please keep entering your guesses until you get the correct answer.");

    for n in 1..=num_iter {
        let mut buffer: String = String::new();

        println!("Enter your guess:");
        stdin_result = io::stdin().read_line(&mut buffer);

        _stdin = match stdin_result {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {e}");
                continue;
            },
        };

        guess_result = buffer.trim().parse();

        guess = match guess_result {
            Ok(g) => g,
            Err(e) => {
                println!("Error: {e}");
                continue;
            },
        };

        if guess > 100 || guess < 1 {
            println!("Your guess of {guess} was not between 1 and 100 inclusive.");
            continue;
        }

        if guess > rand_num {
            println!("Your guess of {guess} was too high.");
        }
        else if guess < rand_num {
            println!("Your guess of {guess} was too low.");
        }
        else {
            println!("Congratulations, you correctly guessed {rand_num} in {n} tries!");
            return;
        }
    }
    println!("Max number of guesses ({num_iter}) exceeded.");
}
