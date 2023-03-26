// https://www.codewars.com/kata/5bb904724c47249b10000131/train/rust

// Our football team has finished the championship.

// Our team's match results are recorded in a collection of strings. Each match is represented by a string in the format "x:y", where x is our team's score and y is our opponents score.

// For example: ["3:1", "2:2", "0:1", ...]

// Points are awarded for each match as follows:

//     if x > y: 3 points (win)
//     if x < y: 0 points (loss)
//     if x = y: 1 point (tie)

// We need to write a function that takes this collection and returns the number of points our team (x) got in the championship by the rules given above.

// Notes:

//     our team always plays 10 matches in the championship
//     0 <= x <= 4
//     0 <= y <= 4

use std::cmp::Ordering;

fn points(games: &[String]) -> u32 {
    games
        .iter()
        .map(|score| {
            let scores = score.split(':').collect::<Vec<_>>();
            match scores[0].cmp(scores[1]) {
                Ordering::Equal => 1,
                Ordering::Greater => 3,
                Ordering::Less => 0,
            }
        })
        .sum()
}
