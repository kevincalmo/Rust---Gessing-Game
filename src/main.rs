use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // Creating a random number between 1 and 10
    let secret_number: u8 = rand::thread_rng().gen_range(1..=10);
    println!("Guess the secret number!");
    loop {
        println!("Please input your guess.");
        //Creation of a mutable variable that can change value during program execution
        let mut guess = String::new();

        //Add input value to the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //Creation of a new guess variable which will necessarily be a number
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, type a numbeer !\n\n");
                continue;
            }
        };
        println!("You guessed: {guess}");

        // check if guess is less than, greater than or equal to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!\n\n"),
            Ordering::Greater => println!("To big!\n\n"),
            Ordering::Equal => {
                println!("You win !\n\n");
                break;
            }
        }
    }
}
