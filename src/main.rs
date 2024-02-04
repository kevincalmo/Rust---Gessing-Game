use rand::Rng;
use std::{cmp::Ordering, io};

fn generate_random_number(min: u8, max: u8) -> u8 {
    return rand::thread_rng().gen_range(min..=max);
}

// function that return anything ( void )
fn gessing_number(secret_number: u8) -> () {
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

fn main() {
    // Creating a random number between 1 and 10
    let secret_number: u8 = generate_random_number(1, 10);
    println!("Guess the secret number!");
    gessing_number(secret_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_number() -> Result<(), String> {
        let min = 1;
        let max = 100;
        let result = generate_random_number(min, max);
        assert_eq!(result, result as u8);
        Ok(())
    }
}