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

        pub fn add_to_waitlist() {
            reserve_table();
            throw_a_tandrum(); // this is above our method directory
        } // crate::front_of_house::hosting::add_to_waitlist()

        fn reserve_table() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
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
