use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::dummy_middleware::DummyMiddleware;
use crate::middlewares::middleware::HttpMiddleware;
use crate::middlewares::routing_middleware::RoutingMiddleware;
use crate::middlewares::logging_middleware::LoggingMiddleware;
use std::cell::Cell;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

pub struct HttpServer {
    routing: Cell<RoutingMiddleware>,
    middlewares: Vec<Box<dyn HttpMiddleware>>,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            routing: Cell::new(RoutingMiddleware::new()),
            middlewares: Vec::new(),
        }
    }

    pub fn run(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    let mut reader = BufReader::new(&_stream);
                    match HttpRequest::from_reader(&mut reader) {
                        Ok(mut request) => {
                            println!(
                                ">> {method} {path}",
                                method = request.method,
                                path = request.path
                            );
                            let response = self
                                .routing
                                .get_mut()
                                .handle(&mut request, &DummyMiddleware::new());
                            _stream
                                .write_all(Vec::<u8>::from(response).as_slice())
                                .unwrap();
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

    pub fn add_route(&mut self, pattern: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routing.get_mut().add_route(pattern, handler);
    }
}
