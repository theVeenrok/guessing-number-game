use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    const MIN_SECRET_NUMBER: u8 = 1;
    const MAX_SECRET_NUMBER: u8 = 255;
    const MAX_ATTEMPTS: u8 = 255;

    println!("Guess the number!");

    let secret_number: u8 = rand::rng().random_range(MIN_SECRET_NUMBER..=MAX_SECRET_NUMBER);
    let mut attempts_count: u8 = 0;

    println!("Please type a number in the range from {MIN_SECRET_NUMBER} to {MAX_SECRET_NUMBER}.");

    loop {
        if attempts_count == MAX_ATTEMPTS {
            println!("You've exhausted your attempts.");
            break;
        }
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let mut user_number: u8 = user_input.trim().parse().expect(
            "Please type a number in the range from {MIN_SECRET_NUMBER} to {MAX_SECRET_NUMBER}!",
        );

        if user_number == 0 {
            println!("Bye-bye...");
            break;
        }
        println!("You guessed: {user_number}");

        match user_number.cmp(&secret_number) {
            Ordering::Less => {
                attempts_count += 1;
                println!("To small!");
            }
            Ordering::Greater => {
                attempts_count += 1;
                println!("To big!");
            }
            Ordering::Equal => {
                attempts_count += 1;
                println!("You Win!");
                break;
            }
        }
    }
    println!("Your attempts: {attempts_count}");
}
