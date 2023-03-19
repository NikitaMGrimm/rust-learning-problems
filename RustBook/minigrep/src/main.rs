use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Arguments can't be parsed: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = minigrep::run(&config) {
        println!("Can't read contents of the file '{file_path}': {err}", 
        file_path = config.file_path,
        );
        process::exit(1);
    };
}



