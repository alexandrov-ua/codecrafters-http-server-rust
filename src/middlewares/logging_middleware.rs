pub struct LoggingMiddleware;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use super::super::middlewares::middleware::HttpMiddleware;


impl LoggingMiddleware {
    pub fn new() -> Self {
        LoggingMiddleware
    }
}

impl HttpMiddleware for LoggingMiddleware {
    fn handle(&self, request: &mut HttpRequest, next: &dyn HttpMiddleware) -> HttpResponse {
        println!("Received request: {} {}", request.method, request.path);
        let response = next.handle(request, next);
        println!("Responding with status: {}", response.status_code());
        response
    }
}