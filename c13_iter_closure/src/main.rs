use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // closure
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        // iterators
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!("#1） Give away the most in the inventory if None color selected.");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("#2） Run an expensive closure (sleeping).");

    let expensive_closure = |num: u32| -> u32 {
        eprint!("Calculating slowly (sleep {num} seconds)...");
        thread::sleep(Duration::from_secs(num as u64));
        eprintln!("{:?}", num);
        num};
    
    let ans = expensive_closure(3);
    println!("Return from expensive_closure is {ans}");

    let mut list = vec![1, 2, 3];
    println!("List before defining & calling closure that modifies the list: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("List after calling closure: {:?}", list);

    println!("#3） Launch a thread to print list.");
    thread::spawn(
        move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    println!("#4) Sort a list of rectangles by width.");

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
        Rectangle { width: 4, height: 5 },
        Rectangle { width: 5, height: 3 },

    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);


}