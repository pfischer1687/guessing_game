use rand::prelude::*;
use std::io;

fn main() {
    let rand_num: i32 = thread_rng().gen_range(1..101);
    let mut guess: i32;
    let num_iter: i32 = 100;
    let mut correct_guess_flag: bool = false;
    
    println!("I have generated a random number between 1 and 100. Please keep entering your guesses until you get the correct answer.");

    for _ in 1..=num_iter {
        let mut buffer: String = String::new();

        println!("Enter your guess:");
        _ = io::stdin().read_line(&mut buffer);
        guess = buffer.trim().parse().unwrap();

        if guess > 100 || guess < 1 {
            println!("Your guess of {guess} was not between 1 and 100 inclusive.");
            continue;
        }

        if guess == rand_num {
            println!("Congratulations, your guess of {guess} was correct!");
            correct_guess_flag = true;
            break;
        }
        else if guess > rand_num {
            println!("Your guess of {guess} was too high.");
        }
        else {
            println!("Your guess of {guess} was too low.");
        }
    }

    if !correct_guess_flag {
        println!("Max number of guesses ({num_iter}) exceeded.")
    }
}
