mod http_request;
mod http_response;
mod some_tests;
mod http_server;
mod url_matcher;
mod middlewares{
    pub mod http_middleware;
    pub mod routing_middleware;
    pub mod logging_middleware;
}
mod http_context;

use crate::http_context::HttpContext;
use crate::http_request::HttpRequest;

extern crate strum;
#[macro_use]
extern crate strum_macros;

fn main() {
    let mut server = http_server::HttpServer::new();

    server.add_route("/hello", |_: &HttpRequest, _: &HttpContext| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
        .with_body("Hello, World!")
    });

    server.add_route("/", |_: &HttpRequest, _: &HttpContext| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
    });

    server.add_route("/echo/{message}", |_: &HttpRequest, context: &HttpContext| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
        .with_body(context.get_path_param("message").unwrap_or(&"".to_string()).as_str())
    });

    server.add_route("/user-agent", |req: &HttpRequest, _: &HttpContext| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
        .with_body(req.headers.get("User-Agent").unwrap_or(&"".to_string()).as_str())
    });

    server.add_route("/delay", |req: &HttpRequest, _: &HttpContext| {
        let delay_seconds: u64 = req.query_params.get("sec")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
            .with_body("Delayed response")
        });
        
    server.run("127.0.0.1:4221");
}