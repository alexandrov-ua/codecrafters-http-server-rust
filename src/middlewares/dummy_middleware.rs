use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use super::super::middlewares::middleware::HttpMiddleware;

pub struct DummyMiddleware;

impl DummyMiddleware {
    pub fn new() -> Self {
        DummyMiddleware{}
    }
}
impl HttpMiddleware for DummyMiddleware {
    fn handle(&self, request: &mut HttpRequest, next: &dyn HttpMiddleware) -> HttpResponse {
        HttpResponse::new(crate::http_response::HttpStatusCode::NotFound)    
    }
}