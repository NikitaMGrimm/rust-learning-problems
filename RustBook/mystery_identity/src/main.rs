fn mystery<T>(x0: T) -> T {
    let mut x1 = x0.clone();
    x1
}

fn main() {
    let y = mystery(3);

    println!("{y:?}");
}
