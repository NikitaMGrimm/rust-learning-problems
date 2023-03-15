fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
    let num = num;
    println!("{}", vec[0]);
    // let num = num; // Won't work because vec doesnt have read perms after mut&.
}
