// this program isn't from codewars. It's from the rust book

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("secret number: {}", secret_number); // debug only

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // num is a placeholder for the thing inside of Ok()
            Err(_) => continue, // Comma isn't needed because it is the last match.
            // Theoretically I CAN use some other name instead of _ but the standard uses _.
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            // I could write {println!("Too big!"")} to leave the comma.
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } // This comma isn't necessary here! Optional!
        }
    }
}
