use std::fmt::Display;
use std::fmt;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger member of {} is x = {}", self, self.x);
        } else {
            println!("The larger member of {} is y = {}", self, self.y);
        }
    }
}

// impl<T: Display> ToString for Pair<T> {
//     fn to_string(&self) -> String {
//         format!("x={}, y={}", self.x, self.y)
//     }
// }

// Between ToString and Display, only one of the traits should be implemented for a struct. 
//  Because implementing Display would automatically implement ToString.
// Implement Display for the structs and for MyTrait
impl<T: Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}