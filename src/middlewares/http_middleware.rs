use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;

pub trait HttpMiddleware {
    fn handle(&self, request: &mut HttpRequest, next: &dyn Fn(&mut HttpRequest) -> HttpResponse) -> HttpResponse;
}