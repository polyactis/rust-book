use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use c20_web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();
    eprint!("Handling request: {request_line}");

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?}", http_request);

    // must add & in front of request_line due to match unable to auto referencing and derefencing.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")},
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    // flush the buffer
    stream.flush().unwrap();
    eprintln!(" Done.");
}

fn main() {
    // [0; 3] is [0,0,0];
    //println!("{:#?}", &[0; 3]);
    let num_threads = 4;
    let num_requests = 3;
    println!("Listening at TCP 127.0.0.1:7878 with {num_threads} threads.");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(num_threads);
    println!("Will gracefully exit after handling {num_requests} requests.");

    for stream in listener.incoming().take(num_requests) {
        let stream = stream.unwrap();

        pool.execute(|| { 
            handle_connection(stream);
        });
    }
}
