use std::borrow::Borrow;
use std::ops::Add;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point{
    x:i32,
    y:i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);
//add a Millimeters with a Meters.
//use references for both Meters and Millimeters so as not to move data (change ownership).
impl Add<&Meters> for &Millimeters{
    type Output = Millimeters;
    fn add(self, other: &Meters) -> Millimeters {
        Millimeters(self.0 + (other.0*1000))
    }
}
pub trait Iterator {
    type Item;
    // self: Refers to the instance of the struct or enum within a method.
    // Self: Refers to the implementing type within associated functions and associated types.
    fn next(&mut self) -> Option<Self::Item>;
}

// mutable global static variable
pub struct COUNTER{
    x:u32,
}

impl Iterator for COUNTER {
    type Item=u32;
    fn next(&mut self) -> Option<Self::Item>{
        Some(self.x)

    }
}
fn main() {
    println!("1) Trait with Associated Types");
    let x = 17;
    println!("x is {x}");
    let mut counter = COUNTER{x:x};
    for i in 1..6{
        let next_one: Option<u32> = counter.next();
        println!("The {i}-th next of counter is {:?}", next_one);

    }

    println!("2) Default generic type parameters and operator overloading");
    let a = Point{x:1, y:0} + Point{x:2, y:3};
    let b = Point{x:3, y:3};
    assert_eq!(
        a,
        b,
    );
    println!("Point a is {:?}.", a);

    println!("2.1) Change default generic type parameter of trait Add to add a Meters to a Millimeters");

    let h_mm = Millimeters(27);
    //let h_mm_rc = Rc::new(&h_mm);
    let h_m = Meters(11);
    let h_total = &h_mm + &h_m;
    println!("Add {:?}mm to {:?}m is {:?}mm.", &h_mm, &h_m, &h_total);

}
