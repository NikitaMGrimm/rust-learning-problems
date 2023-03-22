use std::thread;
use std::time::Duration;

// Channels
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Now lets try to make a channel to pass values between threads.

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // Now we implement shared state concurrency.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Lets try to make a deadlock.
    // one thread wants to lock two variables
    // another one wants the same thing
    // but each of the threads has only one of them

    let var1 = Arc::new(Mutex::new(1));
    let var2 = Arc::new(Mutex::new(2));

    let var1_clone = Arc::clone(&var1);
    let var2_clone = Arc::clone(&var2);

    let handle1 = thread::spawn(move || {
        let mut num1 = var1_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(1));  // we make the thread sleep so that the other thread can lock the other variable
        let mut num2 = var2_clone.lock().unwrap();

        *num1 += 1;
        *num2 += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num2 = var2.lock().unwrap();
        thread::sleep(Duration::from_secs(1)); // we make the thread sleep so that the other thread can lock the other variable
        let mut num1 = var1.lock().unwrap();

        *num1 += 1;
        *num2 += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
