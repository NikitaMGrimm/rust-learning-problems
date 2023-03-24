use std::{io::{BufReader, Read}, fs::File, vec};

fn main() {
    // TESTING STUFF IN THIS CRATE

    // let file = File::open("ASCII_input.txt").unwrap();
    // let mut buf_reader = BufReader::new(file);

    // let mut data = String::new();
    // buf_reader.read_to_string(&mut data).unwrap();

    let vector = vec![1, 2, 3, 4, 5];
    let iter = vector.iter();

    for i in iter().clone() {
        println!("{:#?}", i);
    }

    for i in iter {
        println!("{:#?}", i);
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}