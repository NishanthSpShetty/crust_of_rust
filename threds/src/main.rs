use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use threds::Threadpool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();
    let request_data = String::from_utf8_lossy(&buffer[..]).to_owned();
    println!("\n\nIncoming request\n---\n{}", request_data);

    stream.write_all(status_line.as_bytes()).unwrap();
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    //create a threadpool for executing the each request
    let pool = Threadpool::new(4);

    println!("server is running at port :8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}
