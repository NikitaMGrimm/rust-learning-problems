use std::fmt::Debug;
use std::fmt::Display;

pub trait Greeting {
    fn greet(&self);
    fn default_greeting(&self) {
        println!("Hello!");
    }
}

struct Person {
    name: String,
}

impl Greeting for Person {
    fn greet(&self) {
        println!("Hi, {}!", self.name);
    }

    fn default_greeting(&self) {
        // This code won't compile
        // self.default_greeting();

        println!("Welcome, {}!", self.name);
    }
}

fn hello(hey: &impl Greeting) {
    println!("\nHELLO!");
    hey.greet();
    hey.default_greeting();
}

fn alternative_hello<T: Greeting>(hey: &T) {
    println!("\nHELLO!");
    hey.greet();
    hey.default_greeting();
}

// This is ugly:
fn some_function(t: &(impl Display + Clone), u: &(impl Clone + Debug)) -> i32 {
    0
}

// One variant:
fn alternative0_some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// Other variant:
fn alternative1_some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// We can return types that implement a trait!:
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet(); // Output: Hi, Alice!
    person.default_greeting(); // Output: Welcome, Alice!

    hello(&person);
    alternative_hello(&person);
}
