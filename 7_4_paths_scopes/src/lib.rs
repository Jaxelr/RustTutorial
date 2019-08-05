mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
use self::front_of_house::hosting; //this format will probably be removed on future versions

use std::{cmp::Ordering, io}; //Combine scopes
use std::io::{self, Write}; //Merges std io with std io write
use std::collections::*; //Sample glob operator

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); //notice that we scope to the parent of the func and not the func itself.
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}