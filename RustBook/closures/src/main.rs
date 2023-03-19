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

    // BASICALLY: IMPLEMENT OVERLOAD FOR ABOVE printf THAT ACCEPTS BOTH TYPES OF CLOSURE
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

    /////////////////////////////// CLOSURE PROBLEM QUESTION //////////////////////////////////////
    // Lets try it with a trait. Impl trait on FnOnce, then add it as trait bound to F

    trait DynChainCall {
        // F: FnOnce(i32) -> G,
        // G: FnOnce(i32) -> i32,

        // and type

        // F: FnOnce(i32, i32) -> i32,
        // how?
    }

    impl DynChainCall for FnOnce {
        // ...
    }

    // BASICALLY: IMPLEMENT OVERLOAD FOR ABOVE printf THAT ACCEPTS BOTH TYPES OF CLOSURE
    // THIS DOES NOT WORK EITHER BECAUSE OF : conflicting implementations of trait `FnPrint`

    // trait FnPrint {
    //     fn call_print(&self, a: i32, b: i32) -> i32;
    // }

    // impl<F> FnPrint for F
    // where
    //     F: FnOnce(i32, i32) -> i32,
    // {
    //     fn call_print(&self, a: i32, b: i32) -> i32 {
    //         self(a, b)
    //     }
    // }

    // impl<F, G> FnPrint for F
    // where
    //     F: FnOnce(i32) -> G,
    //     G: FnOnce(i32) -> i32,
    // {
    //     fn call_print(&self, a: i32, b: i32) -> i32 {
    //         self(a)(b)
    //     }
    // }

    // fn printf<P: FnPrint>(f: P, a: i32, b: i32) {
    //     println!("Result is {}", f.call_print(a, b));
    // }

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


// Q on stackoverflow
// https://stackoverflow.com/questions/75785347/how-to-overload-a-rust-function-to-accept-closures-with-different-argument-signa
// I am trying to implement a Rust function `printf` that can accept closures with different argument signatures. The function already has two implementations - `printf1` which accepts closures with two `i32` arguments, and `printf2` which accepts closures with one `i32` argument and returns a closure that also accepts an `i32` argument. <br>(I want to have a `printf` function that chooses between them depending on the input or something similar)

// ```
// fn printf1<F>(f: F, a: i32, b: i32)
// where
// F: FnOnce(i32, i32) -> i32,
// {
// println!("Result is {result}", result = f(a, b),)
// }

// fn printf2<F, G>(f: F, a: i32, b: i32)
// where
// F: FnOnce(i32) -> G,
// G: FnOnce(i32) -> i32,
// {
// println!("Result is {result}", result = f(a)(b),)
// }
// ```

// I tried implementing a trait `DynChainCall` to be able to add the trait as a bound to the `F` type parameter of `printf`. However, I'm not sure how to add support for closures with two `i32` arguments as well as closures that return a closure with a signature of `FnOnce(i32) -> i32`. <br>
// ```
// trait DynChainCall {
//     fn call_print(&self, a: i32, b: i32) -> i32;
// }

// impl<F> DynChainCall for F
// where
//     F: FnOnce(i32, i32) -> i32,
// {
//     fn call_print(&self, a: i32, b: i32) -> i32 {
//         self(a, b)
//     }
// }

// impl<F, G> DynChainCall for F
// where
//     F: FnOnce(i32) -> G,
//     G: FnOnce(i32) -> i32,
// {
//     fn call_print(&self, a: i32, b: i32) -> i32 {
//         self(a)(b)
//     }
// }

// fn printf<P: DynChainCall>(f: P, a: i32, b: i32) {
//     println!("Result is {}", f.call_print(a, b));
// }
// ```
// -> `conflicting implementations of trait DynChainCall`<br><br>
// An example of two closures/functions that I would like to pass into `printf`:
// ```
// fn function(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn func(a: i32) -> impl Fn(i32) -> i32 {
//     move |b| a + b
// }
// ```
// I want `printf` to automatically check if the passed closure `f` accepts one or two arguments and call it accordingly . If the closure accepts one argument and returns a closure with a signature/closure type of `FnOnce(i32) -> i32`, then it should call `f(a)(b)`. If it returns a `i32` then it should call `f(a, b)`.
// ```
// printf(function, 25, 10); // This should work! ( function(25, 10) is called )
// printf(func, 25, 10); // This should also work! ( func(25)(10) is called )
// ```

// How can I implement this behavior of `printf`?