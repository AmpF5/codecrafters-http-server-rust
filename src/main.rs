#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

enum HttpMethods {
    Get,
    Post,
    Update,
    Delete,
}

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
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|f| f.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let data: Vec<&str> = http_request
        .iter()
        .flat_map(|f| f.split_whitespace())
        .collect();
    // let values: Vec<&str> = http_request.iter().flat_map(|f| f).collect();

    println!("{:?}", data);

    let mut response = String::new();

    match data[0] {
        "GET" => {
            let mut url_parts: Vec<&str> = data[1].split('/').collect();
            url_parts.drain(..1);
            url_parts.retain(|s| !s.is_empty());

            if url_parts.is_empty() {
                response = "HTTP/1.1 200 OK\r\n\r\n".into();
            }

            if url_parts.len() == 1 {
                response = "HTTP/1.1 404 Not Found\r\n\r\n".into();
            }
        }
        "POST" => {}
        "UPDATE" => {}
        "DELETE" => {}
        _ => {}
    }

    stream.write_all(response.as_bytes()).unwrap();
}
