// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod front_of_house { // does not need to be public because its a sibling to eat_at_restaurant()
    pub mod hosting {
        use crate::throw_a_tandrum;

        use super::hello_world;

        pub fn add_to_waitlist() {
            reserve_table();
            throw_a_tandrum(); // this is above our method directory
            hello_world();
            super::hello_world(); // both work, why use super though?
            // I guess super:: is for readability and its more useful if we need to go up and back down (super::something::etc..)
        } // crate::front_of_house::hosting::add_to_waitlist()

        fn reserve_table() {
            super::serving::take_payment(); // super goes up to the parent "front_of_house"
        } // super::super or something like that is useless because children can always call up.

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        pub fn take_payment() {}
    }

    fn hello_world() {
        println!("Hello World!");
    }
}

pub fn throw_a_tandrum() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // begin at same level of module tree as eat_at_restaurant()
    self::front_of_house::hosting::add_to_waitlist(); // begin from self
}
