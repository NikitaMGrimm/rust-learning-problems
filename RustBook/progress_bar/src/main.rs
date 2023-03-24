use std::{thread::sleep, time::Duration};

use progress_bar::progress;

fn main() {
    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_calculation);
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
