// use std::io; // Lets try to not use this but do std:: instead.

fn main() {
    loop {
        let mut fahrenheit = String::new();

        println!("Please input a temperature in fahrenheit");

        std::io::stdin()
            .read_line(&mut fahrenheit) // blablabla
            .expect("Failed to read user input"); //

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(f) => f,
            Err(msg) => {
                println!("{msg}\nPlease input a valid number:");
                continue;
            }
        };

        println!(
            "{fahrenheit}F converted into Celsius is {}",
            (fahrenheit - 32.0) * (5.0 / 9.0)
        );
    }
}
