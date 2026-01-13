mod http_request;
use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
mod some_tests;

extern crate strum;
#[macro_use]
extern crate strum_macros;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut reader = BufReader::new(&_stream);
                match http_request::HttpRequest::from_reader(&mut reader) {
                    Ok(request) => {
                        println!(">> {method} {path}", method = request.method, path = request.path);
                        _stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    }
                    Err(e) => {
                        println!("Failed to parse request: {}", e);
                    }
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
