use hello_multithread::ThreadPool;
use std::thread;

fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 0..185 {
        pool.execute(move || {
            fib(i);
        });
    }
}

fn fib(n: i32) {
    thread::sleep(std::time::Duration::from_nanos(1));
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let temp: u128 = a;
        a = b;
        b = temp + b;
    }
    println!("Fibonacci {n}: {}", a);
}
