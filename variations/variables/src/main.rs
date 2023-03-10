const GLOBAL_CONSTANT: u32 = 60 * 39; // this will be calculated at compile time (60*39)
                                      // https://doc.rust-lang.org/reference/const_eval.html
                                      // for more examples
fn main() {
    const MAIN_CONSTANT: i16 = match true {
        true => 123,
        false => 321,
    }; // see link above, match is allowed for const (but what if it evaluates on user input??)
       // lets test it:

    /*
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    const MAIN_CONSTANT_ON_INPUT: u32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    */
    // This WILL NOT work because there is a non-const inside of the match!

    // let x = 5;
    // we need to make x a mutable variable to change it
    let mut x = 5;

    // let x = 5;
    // let mut x = x; // we can "make"(?) a variable mutable (not recommended?) by shadowing.
    // probably not really "making" it mut but only making a new copy that is mut, right?

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("The value of GLOBAL_CONSTANT is: {GLOBAL_CONSTANT}");
    println!("The value of MAIN_CONSTANT is: {MAIN_CONSTANT}");
    // println!("The value of MAIN_CONSTANT is: {MAIN_CONSTANT_ON_INPUT}"); // See above.

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of MAIN_CONSTANT is: {spaces}");
    // this works because we are shadowing a new variable with the same name
    // thus, we can "change" the type of spaces. (but actually its only a new variable)

    // this WONT work if spaces is mutable and we don't use "let"!:
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // this will not work, for example, because we are changing the type
    // this probably doesn't work because types have different sizes in memory
}
