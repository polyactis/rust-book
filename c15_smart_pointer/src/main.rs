#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

use crate::List::{Cons, Nil};
use std::{ops::Deref, rc::Rc};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //println!("{:?}", list);

    let x = 5;
    //let y = &x;
    //let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop(); //explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("strong count of a after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("strong count of a after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("strong count of a after creating c = {}", Rc::strong_count(&a));
    }
    println!("strong count of a after c goes out of scope = {}", Rc::strong_count(&a));
}