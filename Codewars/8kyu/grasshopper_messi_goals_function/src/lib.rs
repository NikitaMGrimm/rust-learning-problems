// https://www.codewars.com/kata/55f73be6e12baaa5900000d4/train/rust

// Messi goals function

// Messi is a soccer player with goals in three leagues:

//     LaLiga
//     Copa del Rey
//     Champions

// Complete the function to return his total number of goals in all three leagues.

// Note: the input will always be valid.

// For example:

// 5, 10, 2  -->  17

fn goals(la_liga_goals: i32, champions_league_goals: i32, copa_del_rey_goals: i32) -> i32 {
    la_liga_goals + champions_league_goals + copa_del_rey_goals
}
