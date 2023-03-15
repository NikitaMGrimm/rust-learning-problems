fn fib(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for i in 0..=20 {
        println!("{}", fib(i));
    }
}
