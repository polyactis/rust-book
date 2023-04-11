use std::fs::File;
use std::io::{ErrorKind, Write};

fn main() {
    println!("Hello, world!");
    let file_path = "hello.txt";
    let greeting_file_result = File::open(file_path);

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file {file_path}: {:?}", e),
            },
            other_error => {
                panic!("Failed to open file {file_path}: {:?}", other_error);
            }
        },
    };
    println!("Writing a Russian word to {file_path}.");
    let written_result = greeting_file.write("Здравствуйте".as_bytes());
    let no_of_bytes_written = match written_result {
        Ok(no_of_bytes_written) => no_of_bytes_written,
        Err(e) => {
            println!("Failed to write a Russian word to {file_path}: '{e}'."); 
            0
        }
    };

    println!("{no_of_bytes_written} bytes written to {file_path}.");
    //greeting_file.flush();

    let v = vec![1, 2, 3];

    v[99];

    panic!("crash and burn");

}
