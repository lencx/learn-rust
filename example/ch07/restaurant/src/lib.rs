mod back_of_house;
mod front_of_house;

// use self::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    hosting::seat_at_table();

    hosting::seat_at_table();


    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order = back_of_house::Appetizer::Soup;
}