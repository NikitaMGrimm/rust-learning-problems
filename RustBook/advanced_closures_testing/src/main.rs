fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice0(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice1(f: impl Fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
enum Status {
    Value(u32),
    Stop,
}

// Try to return a closure:
// fn returns_closure1() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }
// This will not work because it does not have a concrete type.
// Instead do this (for example):
fn returns_closure1() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure2() -> fn(i32) -> i32 {
    move |x| x + 1
}

static hello: i32 = 1;
fn __returns_closure(a: &i32) -> impl Fn(i32) -> i32 + '_ {
    move |x| x + a
}

fn _returns_closure() -> fn(i32) -> i32 {
    move |x| x + hello
}

fn main() {
    let answer1 = do_twice0(add_one, 5);
    let answer2 = do_twice1(add_one, 5);

    println!("The answer is: {}", answer1);
    println!("The answer is: {}", answer2);

    let list_of_statuses1: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let list_of_statuses2: Vec<Status> = (0u32..20).map(|x| Status::Value(x)).collect();  // same thing

    // recommended: use a generic type T and bound it with impl FnOnce/Mut/etc... ->... instead of making it type fn(...)->...
    // can be used in more places. (Use fn() if you need to (for example) call it in C, C does not have closures.)

}
