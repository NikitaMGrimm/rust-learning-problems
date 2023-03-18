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

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet(); // Output: Hi, Alice!
    person.default_greeting(); // Output: Welcome, Alice!

    hello(&person);
}
