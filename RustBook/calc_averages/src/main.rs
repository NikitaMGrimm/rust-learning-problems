use std::collections::HashMap;

fn calculate_median(int_list: &[i32]) -> i32 {
    let mut temp = int_list.to_vec();
    let vec_len = temp.len();
    temp.sort();
    let median_i: usize = vec_len / 2;
    temp[median_i]
} // not reaaaly correct. Avg of middle two if size is even should be standard.

fn calculate_mean(int_list: &[i32]) -> f64 {
    let sum: i64 = int_list.iter().map(|&x| x as i64).sum();
    sum as f64 / int_list.len() as f64
}

fn calculate_mode(int_list: &[i32]) -> i32 {
    let mut int_counts = HashMap::new();
    for &num in int_list {
        *int_counts.entry(num).or_insert(0) += 1;
    }

    int_counts.into_iter()
        .max_by_key(|&(_, val)| val)
        .map(|(num, _)| num)
        .expect("Should be an array with numbers in it.")
}

// Alternative mode:
// fn mode(numbers: &[i32]) -> Option<i32> {
//     let mut counts = HashMap::new();

//     numbers.iter().copied().max_by_key(|&n| {
//         let count = counts.entry(n).or_insert(0);
//         *count += 1;
//         *count
//     })
// }

fn main() {
    let int_list =  vec![-9, 1, 2, 3, 6, 3, 3, 2, 0, -1, 9];
    
    println!("Given: {int_list:?}, median value: {median}, arithmetic mean: {mean}, mode: {mode}",
                median = calculate_median(&int_list),
                mean = calculate_mean(&int_list),  
                mode = calculate_mode(&int_list), 
            );
}
