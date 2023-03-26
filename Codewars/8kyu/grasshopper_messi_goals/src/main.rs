// https://www.codewars.com/kata/55ca77fa094a2af31f00002a/train/rust

// Messi's Goal Total

// Use variables to find the sum of the goals Messi scored in 3 competitions
// Information

// Messi goal scoring statistics:
// Competition 	Goals
// La Liga 	43
// Champions League 	10
// Copa del Rey 	5
// Task

//     Create these three variables and store the appropriate values using the table above:

//     la_liga_goals
//     champions_league_goals
//     copa_del_rey_goals

//     Create a fourth variable named total_goals that stores the sum of all of Messi's goals for this year.

static la_liga_goals: u32 = 43;
static champions_league_goals: u32 = 10;
static copa_del_rey_goals: u32 = 5;

static total_goals: u32 = la_liga_goals + champions_league_goals + copa_del_rey_goals;
