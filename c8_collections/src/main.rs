use std::collections::HashMap;

fn main() {
    //let v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    let third = &v[2];
    println!("The third element is {third}");

    let first = &v[0];

    println!("The first element is: {first}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    for i in &v {
        println!("{i}");
    }
    // Put the mutable vector at the last position. Then it will work.
    v.push(9);

    println!("The v is {:?}.", v);
    // instead of using first here, use v[0] to avoid immutable borrow.
    println!("The first element is: {}", &v[0]);
    for i_ref in &mut v {
        *i_ref += 50;
    }
    println!("The v is {:?}.", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("The row is {:?}.", row);

    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly:
    //let s = "initial contents".to_string();
    //let s = String::from("initial contents");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {:?}.",s3);
    // s1 is no longer valid unless use s1.clone() during s3 construction
    //println!("s1 is {:?}.",s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {:?}.",s);

    let hello = "Здравствуйте";

    let h = &hello[0..4];
    println!("h of hello is {h}.");

    for c in hello.chars() {
        print!("{c} ");
    }
    println!();

    for b in hello.bytes() {
        print!("{b} ");
    }
    println!();

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score for {team_name} is {:?}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores are {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count_ref = map.entry(word).or_insert(0);
        *count_ref += 1;
    }

    println!("{:?}", map);

}
