pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_breakfast() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        println!("Initial Breakfast is {:?}.", meal);
        // Change our mind about what bread we'd like
        meal.toast = String::from("Baguette");
        println!("I'd like {} toast please", meal.toast);
        println!("Breakfast is {:?}.", meal);
    }
}


pub mod front_of_house;
mod back_of_house;
fn deliver_order() {}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();


    front_of_house::hosting::seat_at_table();

    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();

    back_of_house::fix_incorrect_order();
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1 is {:?}. order2 is {:?}!", order1, order2);

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("Breakfast is {:?}.", meal);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    front_of_house::serving::take_payment();

}

