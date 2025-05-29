#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

fn main() {
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|f| f.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //   println!("{:#?}", http_request);
    println!("HTTP/1.1 200 OK\r\n\r\n");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}
