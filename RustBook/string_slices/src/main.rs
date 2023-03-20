fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    // this won't work because its a String, we need &str (pointer with length) :)
    // let _word = first_word(my_string);

    // references are fine too! (because of the implicit .as_bytes()? Does it auto dereference?)
    let _word = first_word(&my_string);
    println!("{_word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(my_string_literal);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    // I can take a reference too! (As often as I like :))
    let _word = first_word(my_string_literal);
    println!("{_word}");

    ///////////////////////////////////////////////////////////////////////////////////////////////

    // This will work.
    let mut s = String::from("hello");
    let o_size = s.as_bytes().len();
    for i in 0..o_size {
        if s.as_bytes()[i] == b'l' {
            s.push_str(" world");
        }
    }
    println!("{s}");

    // But this won't because mutation and aliasing at the same time is occuring.
    // let mut s = String::from("hello");
    // for &item in s.as_bytes().iter() {
    //     if item == b'l' {
    //         s.push_str(" world");
    //     }
    // }
    // println!("{s}");
}
