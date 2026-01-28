use crate::http_context::HttpContext;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;
use crate::middlewares::routing_middleware::RoutingMiddleware;
use std::io::{BufReader, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

pub struct HttpServer {
    routing: Option<RoutingMiddleware>,
    middlewares: Option<Vec<Box<dyn HttpMiddleware + Send + Sync>>>,
}

impl HttpServer {
    pub fn new() -> Self {
        let mut middlewares: Vec<Box<dyn HttpMiddleware + Send + Sync>> = Vec::new();
        HttpServer {
            routing: Some(RoutingMiddleware::new()),
            middlewares: Some(middlewares),
        }
    }

    fn create_middleware_chain(
        vec: Vec<Box<dyn HttpMiddleware + Send + Sync>>,
    ) -> Box<dyn Fn(&mut HttpRequest) -> HttpResponse + Send + Sync> {
        let mut next_fn: Box<dyn Fn(&mut HttpRequest) -> HttpResponse + Send + Sync> =
            Box::new(|_: &mut HttpRequest| {
                HttpResponse::new(crate::http_response::HttpStatusCode::NotFound)
            });
        for mv in vec.into_iter() {
            let current_next = next_fn;
            next_fn = Box::new(move |req: &mut HttpRequest| mv.handle(req, current_next.as_ref()));
        }
        next_fn
    }

    fn handle_connection(
        mut stream: std::net::TcpStream,
        middlewares_chain: &dyn Fn(&mut HttpRequest) -> HttpResponse,
    ) {
        let mut buf_reader = BufReader::new(&mut stream);
        let request = HttpRequest::from_reader(&mut buf_reader).unwrap();

        let mut req = request;
        let response = middlewares_chain(&mut req);

        stream.write_all(&response.to_bytes()).unwrap();
    }

    pub fn run(&mut self, addr: &str) {
        self.middlewares
            .as_mut()
            .unwrap()
            .insert(0, Box::new(self.routing.take().unwrap()));

        let middlewares_chain: Arc<
            Box<dyn Fn(&mut HttpRequest<'_>) -> HttpResponse + Send + Sync>,
        > = Arc::new(HttpServer::create_middleware_chain(
            self.middlewares.take().unwrap(),
        ));

        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    let middlewares_chain = Arc::clone(&middlewares_chain);
                    thread::spawn(move || {
                        HttpServer::handle_connection(_stream, middlewares_chain.as_ref());
                    });
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

    pub fn use_middleware(&mut self, middleware: Box<dyn HttpMiddleware + Send + Sync>) {
        self.middlewares.as_mut().unwrap().push(middleware);
    }
}
