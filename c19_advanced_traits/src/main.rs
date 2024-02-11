use std::fmt;
use std::ops::Add;
use std::thread;
use std::time::Duration;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
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
impl Add<&Meters> for &Millimeters {
    type Output = Millimeters;
    fn add(self, other: &Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
pub trait Iterator {
    type Item;
    // self: Refers to the instance of the struct or enum within a method.
    // Self: Refers to the implementing type within associated functions and associated types.
    fn next(&mut self) -> Option<Self::Item>;
}

// mutable global static variable
pub struct COUNTER {
    x: u32,
}

impl Iterator for COUNTER {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.x)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot speaking fly.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard speaking fly!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlierPrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// this trait requires Point to implement 
impl OutlierPrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// alias for a lengthy type. Note Fn vs fn.
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk){
    f();
}

fn returns_long_type() -> Thunk{
    Box::new(|| println!("hi"))
}

fn main() {
    println!("1) Trait with Associated Types");
    let x = 17;
    println!("x is {x}");
    let mut counter = COUNTER { x: x };
    for i in 1..6 {
        let next_one: Option<u32> = counter.next();
        println!("The {i}-th next of counter is {:?}", next_one);
    }

    println!("2) Default generic type parameters and operator overloading");
    let a = Point { x: 1, y: 10 } + Point { x: 2, y: 3 };
    let b = Point { x: 3, y: 13 };
    assert_eq!(a, b,);
    println!("Point a is {:?}.", a);

    println!(
        "2.1) Change default generic type parameter of trait Add to add a Meters to a Millimeters"
    );

    let h_mm = Millimeters(27);
    //let h_mm_rc = Rc::new(&h_mm);
    let h_m = Meters(11);
    let h_total = &h_mm + &h_m;
    println!("Add {:?}mm to {:?}m is {:?}mm.", &h_mm, &h_m, &h_total);

    println!("3) Fully Qualified Syntax to disambiguate methods with the same name.");

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    (&person as &dyn Pilot).fly();
    (Box::new(person) as Box<dyn Pilot>).fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    println!("4) Supertraits to require one trait's functionality within another trait.");
    println!("Now can print a Point without inheriting Debug trait.");
    println!("Point a {a} outline_print is ");
    a.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world") ] );
    println!("w = {w}");

    println!("5) Alias for a lengthy type.");
    let f: Thunk = Box::new(|| println!("aloha"));
    takes_long_type(f);
    returns_long_type()();
    
    println!("6) DST (Dynamically Sized Type): why str is always &str.");
    // str is a DST (dynamically sized type) and must use a pointer-like (&str slice)
    let s1: &str = "Hellow there!";
    let s2 = "How's it going?";
    println!("s1 is {s1}, s2 is {s2}.");

    println!("7) Never Type: loop/continue/panic! return ! (never type).
    // break returns () (unit type).");
    let mut c = 0;
    // 
    print!("forever ");
    let a = loop {
        c += 1;
        print!("and ever ");
        if c>5 {
            break;
        }
    };
    println!("\n Press Ctrl+C to interrupt the following forever loop.");
    thread::sleep(Duration::from_secs(3));
    let b = loop {
        c += 1;
        print!("bever ");
    };

}
