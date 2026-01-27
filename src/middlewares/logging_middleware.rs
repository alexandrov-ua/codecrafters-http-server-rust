use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;

pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        LoggingMiddleware
    }
}

impl HttpMiddleware for LoggingMiddleware {
    fn handle(&self, request: &mut HttpRequest, next: &dyn Fn(&mut HttpRequest) -> HttpResponse) -> HttpResponse {
        println!("Received request: {} {}", request.method, request.path);
        let response = next(request);
        println!("Responding with status: {}", response.status_code());
        response
    }
}