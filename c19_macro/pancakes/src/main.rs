use hello_macro::HelloMacroTrait;
use hello_macro_derive::HelloMacro;


#[derive(HelloMacro)]
struct Pancakes;


#[derive(HelloMacro)]
struct Icecakes;
fn main() {
    println!("HelloMacro is a custom derivable macro that automatically implements trait HelloMacroTrait on a struct.");
    Pancakes::hello_macro();
    Icecakes::hello_macro();
}
