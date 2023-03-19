use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Arguments can't be parsed: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprintln!(
            "Can't read contents of the file '{file_path}': {err}",
            file_path = config.file_path,
        );
        process::exit(1);
    };
}
