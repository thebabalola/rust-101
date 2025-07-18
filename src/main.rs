// use std::io;
use rand::Rng;
use std::cmp::Ordering;

use colored::*; // Import the colored crate for colored output

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number); 

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
            //or just Err(_) => continue,
        };

        println!("You guessed {}", guess);

        // match guess.cmp(&secret_number.to_string()) // coverts int to string{
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
       
    }

}
