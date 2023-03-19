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

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // Removing "move" above yields this error:
    //     thread::spawn( || println!("From thread: {:?}", list))
    //    |                    ^^                               ---- `list` is borrowed here
    //    |                    |
    //    |                    may outlive borrowed value `list`
    // the thread might need longer to finish than the owner of list without the move!
}
