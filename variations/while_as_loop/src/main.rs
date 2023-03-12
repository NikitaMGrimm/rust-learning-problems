fn main() {
    // while loop without using while
    // while(condition) {...} should execute the code in the block {...}
    // for as long as the condition is true,  if the condition ceases to be true, call a break.

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
}