// To define a struct, we enter the keyword struct and name the entire struct.
// A struct’s name should describe the significance of the pieces of data being grouped together.
// Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
// For example, Listing 5-1 shows a struct that stores information about a user account.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple.
// For example, here we define and use two tuple structs named Color and Point:

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
