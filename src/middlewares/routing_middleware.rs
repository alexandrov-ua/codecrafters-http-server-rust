use std::collections::HashMap;
use super::super::http_request::HttpRequest;
use super::super::http_response::{HttpResponse, HttpStatusCode};
use crate::url_matcher::UrlMatcher;
use crate::middlewares::middleware::HttpMiddleware;

pub struct RoutingMiddleware{
    routes: HashMap<UrlMatcher, fn(&HttpRequest) -> HttpResponse>,
}

impl RoutingMiddleware {
    pub fn new() -> Self {
        RoutingMiddleware{
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, pattern: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        let matcher = UrlMatcher::new(pattern);
        self.routes.insert(matcher, handler);
    }
}

impl HttpMiddleware for RoutingMiddleware {
    fn handle(&self, request: &mut HttpRequest, next: &dyn HttpMiddleware) -> HttpResponse {
        for (matcher, handler) in &self.routes {
            let (matched, params) = matcher.match_url(&request.path);
            if matched {
                return handler(request);
            }
        }
        HttpResponse::new(HttpStatusCode::NotFound)
        //Do not call next in routing middleware
        //next.handle(request)
    }
}
