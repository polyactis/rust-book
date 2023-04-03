use std::io;


fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is: {spaces}. (End of Line)");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}.");
    
    //let guess: u32 = "42".parse().expect("Not a number!");
    let guess = 98_222u32;
    let guess_b = 0b1111_0000;
    let guess_hex = 0o77;
    println!("The guess is: {guess}. guess_b is {guess_b}. guess_hex is {guess_hex}.");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // addition
    //let sum = 5 + 10;

    // subtraction
    //let difference = 95.5 - 4.3;

    // multiplication
    //let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is {quotient}.");

    let truncated = -5 / 3; // Results in -1
    println!("truncated is {truncated}.");

    // remainder
    let remainder = 43 % 5;
    println!("remainder is {remainder}.");
    
    //let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    println!("z is {z}.");
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat is {heart_eyed_cat}.");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is: {:?}", tup);
    println!("tup is: {:#?}", tup);
    let five_hundred = tup.0;
    let one = tup.2;
    println!("five_hundred and one are: {}, {:#?}", five_hundred, one);

    //let (x, y, z) = tup;
    //println!("The value of y is: {y}");

    let tup = ();
    println!("empty tup is {:?}.", tup);
    println!("empty tup is {:#?}.", tup);

    let a = [1, 2, 3, 4, 5];
    //let a = [3; 5];
    println!("array a is {:?}.", a);
    
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    println!("Months are {:?}.", months);
    
    println!("Hello, world!");

    another_function(element);
    print_labeled_measurement(element, heart_eyed_cat);

    let x = five();
    let x = plus_one(x);

    println!("The value of x is: {x}");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The counter result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    //let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    //let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    //let word = first_word(&my_string_literal[0..6]);
    //let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}