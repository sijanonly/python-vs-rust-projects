use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");

    let secret_number = rand::rng().random_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Your guess must be between 1 and 100");
            continue;
        }

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Tow low! Try again."),
            Ordering::Greater => println!("Tow high! Try again."),
            Ordering::Equal => {
                println!("Correct! You guessed it!");
                println!("The number was {secret_number}.");
                println!("You got it in {attempts} attempts.");
                break;
            }
            
        }
    }

}
