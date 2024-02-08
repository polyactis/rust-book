//use core::slice;
use std::slice;


unsafe fn dangerous() {
    println!("Danger!");
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid<=len);
    unsafe {
        ( 
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
    //(&mut values[..mid], &mut values[mid..])
}

/*
- "C" defines ABI the external function uses.
- ABI defines how to call the function at the assembly level.
- The "C" ABI is the most common and follows the C programming language's ABI.
 */
extern "C" {
    // names and signatures of external functions from C
    fn abs(input: i32) -> i32;
}


#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Output from a Rust function! Callable by C.");
}

// immutable global static variable
static HELLO_WORLD: &str = "Hello world!";

// mutable global static variable
static mut COUNTER: u32 = 0;
fn add_to_counter(inc: u32) {
    // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
    // so must be within unsafe block.
    unsafe {
        COUNTER += inc;
    }
}

//unsafe traits
unsafe trait Foo {
    // methods
}

unsafe impl Foo for i32 {
    // 
}

fn main() {
    println!("1) Dereference two raw pointers that point to the same data, output after changing value for one of them.");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // a random address in memory
    let address = 0x012345usize; 
    let r = address as *mut i32;

    unsafe {
        //can change r2 but not r1 because r1 is const raw pointer.
        *r2 += 1;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // accessing this random address results in segmentation fault
        //println!("r is: {}", *r);
    }

    println!("2) Obtain 10K elements at address {address} results in segmentation fault");
    let values= unsafe {
        // appending ; results in returning unit (empty structure)
        // removing ; will incur segmentation fault as it tries to access a random address.
        slice::from_raw_parts_mut(r, 10000);
    };
    println!("10K elements (fake obtaining. true obtaining results in segmentation fault) are {:?}", values);

    println!("3) Call an unsafe function");
    unsafe { 
        dangerous();
    }

    println!("4) Create a safe abstraction over unsafe code");

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);

    println!("First 3 elements are {:?}", a);
    println!("Elements after first 3 elements are {:?}", b);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    println!("4.1) Safely call own split_at_mut() that contains an unsafe block");

    let (a,b) = split_at_mut(&mut v, 3);
    println!("First 3 elements are {:?}", a);
    println!("Elements after first 3 elements are {:?}", b);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    println!("5) Use extern functions to call external code");
    unsafe {
        println!("Absolute value of -3 according to C is {}", abs(-3));
    }

    println!("6) Accessing or changing a global or static variable");
    println!("name is {}", HELLO_WORLD);

    unsafe {
        // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
        // so must be within unsafe block.
        println!("Counter is {COUNTER}");
    }
    add_to_counter(3);
    unsafe {
        // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
        // so must be within unsafe block.
        println!("Counter after adding 3 is {COUNTER}");
    }

}
