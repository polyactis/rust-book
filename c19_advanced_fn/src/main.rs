fn add_one(x: i32) -> i32 {
    x+1
}


fn do_twice(f: fn(i32) -> i32, arg:i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status{
    Value(u32),
    Stop,
}

// return an `impl Trait` instead of a `dyn Trait`, if all returned values are the same type
fn return_closure() -> impl Fn(i32) -> i32{
    |x| x+1
}

// box the return type, and wrap all of the returned values in `Box::new`
// if all returned values are NOT the same type.
fn return_closure2() -> Box<dyn Fn(i32) -> i32>{
    Box::new(|x| x+1)
}

fn main() {
    println!("1) Function pointers.");
    let x  = 5;
    let answer = do_twice(add_one, x);
    println!("The answer of doing twice of adding 1 to {x} is {answer}");

    println!("2) Use either closure or a named function as argument.");
    let ls_of_numbers = vec![1,2,3];
    let ls_of_strings: Vec<String> = ls_of_numbers.iter().map(|i| i.to_string()).collect();
    let ls_of_strings_2: Vec<String> = ls_of_numbers.iter().map(ToString::to_string).collect();

    println!("ls_of_numbers: {:?}", ls_of_numbers);
    println!("ls_of_strings: {:?}", ls_of_strings);
    println!("ls_of_strings_2: {:?}", ls_of_strings_2);

    let ls_of_status: Vec<Status> = (0u32..9).map(Status::Value).collect();
    println!("ls_of_status: {:?}", ls_of_status);

    println!("3) Return closure.");

    let c_a = return_closure();
    println!("c_a(3)={}", c_a(3));

    let c_b = return_closure2();
    println!("c_b(3)={}", c_b(3));
}
