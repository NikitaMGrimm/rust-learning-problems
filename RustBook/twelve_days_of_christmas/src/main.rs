// This will print the lyrics to "The Twelve Days of Christmas", (problem from the rust book).
fn main() {
    let song = [
        ("first", "A partridge in a pear tree"),
        ("second", "Two turtle doves, and"),
        ("third", "Three french hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five golden rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    for verse in 0..12 {
        println!("[Verse {}]", verse + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me",
            song[verse].0
        );
        for line in 0..=verse {
            println!("{}", song[line].1);
        }
        println!();
    }
}
