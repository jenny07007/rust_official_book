fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }

    fn fix_incorrect_order() {
        cook_order();
        // super -> is like starting a filesystem path with the `..` syntax
        super::deliver_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
