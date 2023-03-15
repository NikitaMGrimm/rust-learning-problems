fn main() {
    // while loop without using while
    // while(condition) {...} should execute the code in the block {...}
    // for as long as the condition is true,  if the condition ceases to be true, call a break.
    let condition = false;
    loop {
        if condition {
            // {...}
        } else {
            break;
        }
    }

    // other nesting structure:

    loop {
        if !condition {
            break;
        }
        // {...}
    }

    // short for loop test
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
