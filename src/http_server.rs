use crate::http_context::HttpContext;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;
use crate::middlewares::logging_middleware::LoggingMiddleware;
use crate::middlewares::routing_middleware::RoutingMiddleware;
use std::io::{BufReader, Write};
use std::net::TcpListener;

pub struct HttpServer {
    routing: Option<RoutingMiddleware>,
    middlewares: Vec<Box<dyn HttpMiddleware>>,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            routing: Some(RoutingMiddleware::new()),
            middlewares: Vec::new(),
        }
    }

    fn middleware_chain<'a>(
        &self,
        mut iter: std::slice::Iter<'a, Box<dyn HttpMiddleware>>,
        request: &mut HttpRequest,
    ) -> HttpResponse {
        if let Some(mw) = iter.next() {
            let next_fn: Box<dyn Fn(&mut HttpRequest) -> HttpResponse> = Box::new(|req: &mut HttpRequest| {
                self.middleware_chain(iter.clone(), req)
            });
            mw.handle(request, next_fn.as_ref())
        } else {
            HttpResponse::new(crate::http_response::HttpStatusCode::NotFound)
        }
    }

    pub fn run(&mut self, addr: &str) {
        self.middlewares.push(Box::new(LoggingMiddleware::new()));
        self.middlewares.push(Box::new(self.routing.take().unwrap()));


        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    let mut reader = BufReader::new(&_stream);
                    match HttpRequest::from_reader(&mut reader) {
                        Ok(mut request) => {
                            let iter = self.middlewares.iter();

                            let response = self.middleware_chain(iter, &mut request);

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

    pub fn add_route(
        &mut self,
        pattern: &str,
        handler: fn(&HttpRequest, &HttpContext) -> HttpResponse,
    ) {
        self.routing.as_mut().unwrap().add_route(pattern, handler);
    }
}
