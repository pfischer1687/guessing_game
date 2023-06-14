use rand::prelude::*;
use std::io;

fn main() {
    let rand_num: i32 = thread_rng().gen_range(1..101);
    let mut guess: i32;
    let num_iter: i32 = 5;
    let mut correct_guess_flag: bool = false;
    let mut _stdin: usize;
    
    println!("I have generated a random number between 1 and 100. Please keep entering your guesses until you get the correct answer.");

    for n in 1..=num_iter {
        let mut buffer: String = String::new();

        println!("Enter your guess:");
        _stdin = io::stdin().read_line(&mut buffer).expect("Failed to read the input line.");
        guess = buffer.trim().parse().unwrap();

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
            println!("Congratulations, you correctly guessed {guess} in {n} tries!");
            correct_guess_flag = true;
            break;
        }
    }

    if !correct_guess_flag {
        println!("Max number of guesses ({num_iter}) exceeded.")
    }
}
