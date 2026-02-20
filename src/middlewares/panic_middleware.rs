use std::panic::{self,AssertUnwindSafe};



use crate::http_request::HttpRequest;
use crate::http_response::{HttpResponse, HttpStatusCode};
use crate::middlewares::http_middleware::HttpMiddleware;


pub struct PanicMiddleware;

impl PanicMiddleware {
    pub fn new() -> Self {
        PanicMiddleware
    }
}

impl HttpMiddleware for PanicMiddleware {
    fn handle(
        &self,
        req: &mut HttpRequest,
        next: &dyn Fn(&mut HttpRequest) -> HttpResponse,
    ) -> HttpResponse {
        let result = panic::catch_unwind(AssertUnwindSafe(|| next(req)));
        match result {
            Ok(response) => response,
            Err(_) => HttpResponse::new(HttpStatusCode::InternalServerError)
                .with_body("Internal Server Error"),
        }
    }
}