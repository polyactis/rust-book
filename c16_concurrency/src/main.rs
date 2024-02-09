use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

fn main() {
    println!("1) Create new threads with spawn()");
    let handler1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("3) Use move Closures with Threads. Move v into thread.");
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    println!("2) Waiting for all threads to finish by join()");
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler1.join().unwrap();
    println!("2.1) Prior threads should finish by now");

    println!("3) Channel/Message passing (multi-producers) to transfer data between threads.");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("t1: hi");
        tx1.send(val).unwrap();
        tx1.send(3.to_string()).unwrap();

        let vals = vec![
            String::from("t1: hi"),
            String::from("t1: from"),
            String::from("t1: the"),
            String::from("t1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("t2: more"),
            String::from("t2: messages"),
            String::from("t2: for"),
            String::from("t2: you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    println!("Got: {}", rx.recv().unwrap());
    //println!("Got: {}", rx.recv().unwrap());    //unwrap() will fail due to an empty result.
    
    println!("Printing all received via an iterator...");
    for received in rx {
        println!("Got: {}", received);
    }

    println!("4) Use Arc<Mutex<T>> to modify the same data across threads. Similar to RefCell<T>/RC<T> in single-thread.");
    let counter = Arc::new(Mutex::new(0));
    println!("Counter = {:?}", &counter);
    let mut handles = vec![];

    for _ in 0..10 {
        // increase counter by 1 in each thread
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter = {:?}", &counter);
    println!("Final counter result: {}", *counter.lock().unwrap());

}
