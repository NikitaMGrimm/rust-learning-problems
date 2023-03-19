use std::fmt::Display;
use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("{list:?}"); // Does not work.
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Force a move like this: (needed for threads as main thread might finish first & dropping it)
    let list = vec![1, 2, 3];
    println!("\nThread:\nBefore defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?} \n\n", list))
        .join()
        .unwrap();
    // Removing "move" above yields this error:
    //     thread::spawn( || println!("From thread: {:?}", list))
    //    |                    ^^                               ---- `list` is borrowed here
    //    |                    |
    //    |                    may outlive borrowed value `list`
    // the thread might need longer to finish than the owner of list without the move!

    // Lets try to rewrite a function(i32, i32) -> i32 into
    // function(i32) -> f(i32) -> i32
    // Because it reminded me of haskell o.o

    /////////////////////////////// CLOSURE PROBLEM QUESTION //////////////////////////////////////

    fn printf1<F>(f: F, a: i32, b: i32)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        println!("Result is {result}", result = f(a, b),)
    }

    fn printf2<F, G>(f: F, a: i32, b: i32)
    where
        F: FnOnce(i32) -> G,
        G: FnOnce(i32) -> i32,
    {
        println!("Result is {result}", result = f(a)(b),)
    }

    // BASICALLY: IMPLEMENT OVERLOAD FOR ABOVE printf THAT ACCEPTS BOTH TYPES OF CLOSURE

    trait FnPrint {
        type Output;
        fn call_print(&self, a: i32, b: i32) -> i32;
    }
    
    impl<F> FnPrint for F
    where
        F: FnOnce(i32, i32) -> i32,
    {
        fn call_print(&self, a: i32, b: i32) -> i32 {
            self(a, b)
        }
    }
    
    impl<F, G> FnPrint for F
    where
        F: FnOnce(i32) -> G,
        G: FnOnce(i32) -> i32,
    {
        fn call_print(&self, a: i32, b: i32) -> i32 {
            self(a)(b)
        }
    }
    
    fn printf<P: FnPrint>(f: P, a: i32, b: i32) {
        println!("Result is {}", f.call_print(a, b));
    }
    
    /////////////////////////////// CLOSURE PROBLEM QUESTION //////////////////////////////////////

    fn function(a: i32, b: i32) -> i32 {
        a + b
    }

    fn func(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    printf(function, 25, 10);
    // I want printf to work on func too :)
    // For now I need to do this:
    println!("Result is {result}", result = func(25)(10));
    // I can ofc make this into another method but:
    // printf(func, a1, b1); should work. printf should automatically
    // check if the passed closure accepts one or two arguments
    // if it accepts one argument and it return closure with type
    // "impl Fn(i32) -> i32" then it should call f(a1)(b1)
    // else, it should panic or something i dont care
}
