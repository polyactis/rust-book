use std::fs::File;
use std::io::{self, Read, ErrorKind, Write};

fn main() {
    let file_path = "hello.txt";
    let greeting_file_result = File::open(file_path);

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file_created) => file_created,
                Err(e) => panic!("Failed to create file {file_path}: {:?}", e),
            },
            other_error => {
                panic!("Failed to read file {file_path}: {:?}", other_error);
            }
        },
    };

    println!("Going to write a Russian word to {file_path}.");
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

    let greeting_file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file {file_path}: {:?}", error);
            })
        } else {
            panic!("Problem reading file {file_path}: {:?}", error);
        }
    });

    println!("Content in {file_path} is {}.", read_username_from_file(file_path).unwrap());

    let v = vec![1, 2, 3];

    v[99];

    panic!("crash and burn");

}



fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}