
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

#[derive(Debug)]
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
}

pub fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}