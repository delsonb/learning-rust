mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// by deafult the items brought into scope with "use" are private
// to make them available as if they had been defined in this scope,
// we can combine "pub" and "use".
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
