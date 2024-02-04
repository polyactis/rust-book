fn print_coordinates(&(x,y): &(i32, i32)){
    println!("Current location: ({}, {})", x,y);

}
fn main() {
    println!("1) Increment x by 1 if it is not None!");
    let mut x = Some(1);
    //let mut x: Option<i32>  = None;
    println!("x={:?}", x);
    //let x = ();
    x = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("x={:?}", x);

    println!("2) Determine the BG color according to many conditions via if let.");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color as age>30");
        } else {
            println!("Using orange as the background color as age<=30");
        }
    } else {
        println!("Using blue as the background color");
    }

    println!("3) while let pattern.");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    println!("4) for loops");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in  v.iter().enumerate(){
        println!("Value {value} at index {index}");
    }

    println!("5) Patterns in function parameters");
    let point = (3,5);
    print_coordinates(&point);

    // refutable pattern disallowed with let.
    //let Some(x) = Some(10);

    println!("6) Matching literal patterns");
    let x =1;
    match x{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("7) Matching named variables");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y={y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("At the end: x={:?}, y={y}", x);

    println!("8) Multiple patterns");
    let x = 2;
    match x{
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("9) Matching ranges of values with ..=");
    let x = 1;
    match x {
        1..=5 => println!("One through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("an early ASCII letter"),
        'k'..='z' => println!("a late ASCII letter"),
        _ => println!("something else"),

    }

    println!("10) De-structure structs");
    struct Point{
        x:i32,
        y:i32,
        z:i32,
    }
    let p = Point{x:0, y:7, z:-1};
    let Point {x:a, y:b, z:c} = p;
    assert_eq!(a, 0);
    assert_eq!(7, b);
    assert_eq!(-1, c);

    assert!(a==0);

    //shorthand form
    let Point {x, y, z} = p;
    assert_eq!(x, 0);
    assert_eq!(7, y);
    assert_eq!(z, -1);
    assert!(x==0);

    println!("10.1) De-structure structs with literal values");
    match p {
        Point{x, y:0, z:-1} => println!("One the x axis is {x}"),
        Point{x:0, y, z} => println!("One the y axis is {y}"),
        Point{x, y, z} => {
            println!("On neighter axis: ({x}, {y}, {z})");
        }
    }

    println!("11) De-structure enums");
    
    enum Message{
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160 , 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, greeen {g}, and blue {b}");
        }
        
    }
    
    println!("12) De-structure nexted structs and enums");
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2{
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160 , 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("Change the color to red {r}, greeen {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, value {v}");

        }
        _ => (),
    }

    println!("13) De-structure nested structs and tuples");
    let ((feet, inches), Point { x, y, z }) = ((3,10), Point{x:3, y:-10, z:5});
    println!("{feet}-feet, {inches}-inches, Point x={x}, y={y}, z={z}");


    println!("14) Ignore parts of a value with a nested _");
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }

    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: 1st={first}, 3rd={third}, 5th={fifth}");
        }
    }
    println!("15) Ignore remaining parts of a value ..");
    let origin = Point{x:0, y:0, z:0};
    match origin{
        Point{x, ..} => println!("x is {x}"),
    }

    println!("16) Match Guard ..");
    let num = Some(4);
    match num {
        Some(x) if x%2==0 => println!("number {x} is even."),
        Some(x) => println!("number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n==y => println!("Matched, n={n}"),
        _ => println!("Default case, x={:?}", x),
    }
    println!("At the end: x={:?}, y={y}", x);

    println!("17) @ Bindings ..");
    enum Message3 {
        Hello {id: i32},
    }
    let msg = Message3::Hello {id: 5};
    match msg {
        Message3::Hello{id: id_variable @ 3..=7, } => 
            println!("Found an id in range [3-7]: {id_variable}"),
        Message3::Hello{id: 10..=12} => println!("Foudn an id in another range"),
        Message3::Hello{id} => println!("Found some other id: {id}"),
    }

}
