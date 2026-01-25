use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;

pub struct DummyMiddleware;

impl DummyMiddleware {
    pub fn new() -> Self {
        DummyMiddleware{}
    }
}
impl HttpMiddleware for DummyMiddleware {
    fn handle(&self, _: &mut HttpRequest, _: &dyn Fn(&mut HttpRequest) -> HttpResponse) -> HttpResponse {
        HttpResponse::new(crate::http_response::HttpStatusCode::NotFound)    
    }
}