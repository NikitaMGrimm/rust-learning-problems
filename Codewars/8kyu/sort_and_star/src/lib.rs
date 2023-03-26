// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/train/rust

// You will be given a list of strings. You must sort it alphabetically (case-sensitive, and based on the ASCII values of the chars) and then return the first value.

// The returned value must be a string, and have "***" between each of its letters.

// You should not remove or add elements from/to the array.

fn two_sort(arr: &[&str]) -> String {
    let min = arr.iter().min().unwrap();
    min.chars()
        .map(|c| format!("{}***", c))
        .collect::<String>()
        .trim_end_matches("***")
        .to_string()
}

fn two_sort2(arr: &[&str]) -> String {
    let min = arr.iter().min().unwrap();
    min.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("***")
}

