use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    /*  let mut choice = String::new();
    println!("What do you want?\nType 1 for Guess Game;\nType -1 to exit;");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line :/");

    println!("Your choice was >>> {choice}"); */

    let secret_number = rand::thread_rng().gen_range(1..=128);
    //println!("The secret number is: {secret_number}");

    //      # Chapter 2 finished!!!
    loop {
        let mut guess = String::new();
        println!("\nGuess a number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("--------------\n|            |\n| YOU WIN!!! |\n|            |\n--------------");
                break;
            }
        }
    }

    /*  if  {
        println!("Exiting...");
    } else if choice == "1" {
        println!("That the game begin!");

        let secret_number = rand::thread_rng().gen_range(1..=128);
        let mut guess = String::new();

        println!("The secret number is: {secret_number}");

        println!("Guess a number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :/");

        println!("You guessed: {guess}");
    } else {
        println!("No one");
    } */
}
