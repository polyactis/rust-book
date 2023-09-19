#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    println!("Hello, world!");
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    println!("x is {}.", x);
    // The next print will error as mutable and immutable borrows are 
    //  prohibited to exist simultaneously.
    //println!("y is {}.", *y);

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    *value.borrow_mut() += 10;
    println!("10 is now added to value.");
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
