mod front_of_house;

use crate::back_of_house::Appetizer;
pub use crate::front_of_house::hosting;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


fn deliver_order() {}

mod back_of_house {
    // -- Breakfast --
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // -- Appetizer --
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    hosting::add_to_waitlist(); // relative path

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);

    println!("What Appetizer do you want?  {:?} or {:?}", Appetizer::Salad, Appetizer::Soup)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
