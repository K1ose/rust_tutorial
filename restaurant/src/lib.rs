fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        use super::serving;

        pub fn add_to_waitlist() {
            println!("You add `pub` to my module and my function.");
            println!("Welcome to our restaurant, you have been added to waitlist!");
            println!("Now, You can seat at table!");
            seat_at_table();
        }

        fn seat_at_table() {
            serving::take_order();
        }
    }

    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    mod food_package {
        pub struct Breakfast {
            pub toast: String, // `pub` here
            seasonal_fruit: String,
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }

            pub fn spring(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("oranges"),
                }
            }
            pub fn autumn(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("apples"),
                }
            }
            pub fn winter(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("strawberries"),
                }
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist(); // If the prefix of `hosting` does not include 'pub', the module `hosting` is private. If the prefix of `add_to_waitlist()` does not include 'pub', the function `add_to_`waitlist()` is private.

    // 相对路径
    // front_of_house::hosting::add_to_waitlist(); // module `hosting` is private private module
    // let mut meal = back_of_house::food_package::Breakfast::winter("Rye");

    // // 修改toast
    // meal.toast = String::from("wheat");
    // println!("I'd like {} toast please.", meal.toast);

    // let order_a_salad = back_of_house::food_package::Appetizer::Salad;
    // let order_a_soup = back_of_house::food_package::Appetizer::Soup;

    hosting::add_to_waitlist();
}

/// a usaul usage
use std::collections::HashMap as HashMap;

/// a usaul usage of HashMap
fn use_usage() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

pub use crate::front_of_house::hosting;
// Original code
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
