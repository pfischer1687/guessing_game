use rand::prelude::*;
use std::io;
use std::num::ParseIntError;

fn main() {
    
    let mut stdin_result: Result<usize, io::Error>;
    let mut _stdin: usize;
    let mut guess_result: Result<i32, ParseIntError>;
    let mut _guess: i32;
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

        let _guess = match guess_result {
            Ok(g) => {
                if g > 100 || g < 1 {
                    println!("Your guess of {g} was not between 1 and 100 inclusive.");
                    continue;
                }

                if g > rand_num {
                    println!("Your guess of {g} was too high.");
                    continue;
                }
                else if g < rand_num {
                    println!("Your guess of {g} was too low.");
                    continue;
                }
                else {
                    println!("Congratulations, you correctly guessed {rand_num} in {n} tries!");
                    return;
                }
            }
            Err(e) => {
                println!("Error: {e}");
                continue;
            },
        };
    }

    println!("Max number of guesses ({num_iter}) exceeded.");
    return;
}
