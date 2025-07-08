use std::io; // For handling input
use rand::Rng; // For generating random numbers
use std::cmp::Ordering; // For comparing values

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("(Debug) The secret number is: {secret_number}"); // Uncomment to cheat 😄

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert input to number, handle error if not a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! 📉"),
            Ordering::Greater => println!("Too big! 📈"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;
            }
        }
    }
}
