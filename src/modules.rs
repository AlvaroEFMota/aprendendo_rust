mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Eating in a restaurant");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::modules::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//------------------------------------------------------------------------

#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), // Only in methods we can change the value of seasonal_fruit
            }
        }
    }
}

//------------------------------------------------------------------------

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    // *We cannot create an instance of Breakfast without the summer
    // function because we can't set a value to the private atribute seasonal_fruit
    // only the method summer can set it*
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

//----------------------------------------------------------------------------

mod back_of_house3 {
    pub enum Appetizer { // if we make an enum public, all of its variants are then public.
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() {
    let _order1 = back_of_house3::Appetizer::Soup;
    let _order2 = back_of_house3::Appetizer::Salad;
}

//----------------------------------------------------------------------------

mod front_of_house4 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// hosting is now a valid name in this scope.
// We can also bring a function with use keyword, like
// use crate::modules::front_of_house4::hosting::add_to_waitlist;
use crate::modules::front_of_house4::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//----------------------------------------------------------------------------

/**
 * The 'as' keyword allow us to bring two equals names to the main scope and turn one to a new name, like
 * use std::fmt::Result;
 * use std::io::Result as IoResult;
 */


/**
 * Instead use this
 * use std::cmp::Ordering;
 * use std::io;
 * 
 * We can use this
 * use std::{cmp::Ordering, io};
 */

/**
 * Instead use this
 * use std::io;
 * use std::io::Write;
 * 
 * We can use this
 * use std::io::{self, Write};
 */

/**
 * // To bring all public items, we use '*'
 * use std::collections::*;
 */
//----------------------------------------------------------------------------


pub fn run() {
    eat_at_restaurant();
    eat_at_restaurant2();
    eat_at_restaurant3();
    eat_at_restaurant4();
    
}
