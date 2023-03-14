#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
    area: u32,
}

fn init_rectangle(width: u32, height: u32) -> Rect {
    Rect {
        width,
        height,
        area: width * height,
    }
}

fn main() {
    let rectangle = init_rectangle(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area,
    );

    println!(
        "Lets try to print the rectangle with debug :? -> {:?}",
        rectangle,
    );

    println!(
        "Lets try this again but with :#? -> {:#?}",
        rectangle,
    );

    // Lets try the dbg!() macro
    dbg!(&rectangle);
}
