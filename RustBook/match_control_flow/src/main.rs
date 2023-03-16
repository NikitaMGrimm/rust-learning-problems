#[derive(Debug)]
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn print_range_max(loc: &Location) {
    // print the second field of Range, if loc is a Range
    if let Location::Range(_, y) = loc {
        println!("{}", y);
    }
}

fn get_start(loc: &Location) -> i32 {
    match loc {
        Location::Point(x) => *x,
        Location::Range(x, _) => *x,
    }
    // return the first field of Range or the only field of Point
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // This can be written like:

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let borrow_test = Location::Range(2, 22);
    print_range_max(&borrow_test);

    println!("{borrow_test:?}");

    let y = if let Location::Range(_, y) = borrow_test { y }
    else {0};
    println!("{}", y);
}