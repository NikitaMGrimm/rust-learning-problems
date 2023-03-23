fn is_equal0<T: Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
}

fn is_equal1<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
    t1 == t2
}

fn _is_equal1_deref<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
    *t1 == *t2  // implicitely reference/deref to ::std::ops::PartialEq::eq(&t1, &t2) so it works.
}

fn is_equal2<T: Eq>(t1: T, t2: T) -> bool {
    t1 == t2
}

fn main() {
    println!("{:?}", is_equal0(&"Hello", &"world"));

    println!("{:?}", is_equal1("Hello", "world"));

    println!("{:?}", is_equal2("Hello", "world"));
    println!("{:?}", is_equal2(&"Hello", &"world"));

}

