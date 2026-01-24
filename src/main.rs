mod http_request;
mod http_response;
mod some_tests;
mod http_server;
mod url_matcher;
mod middlewares{
    pub mod middleware;
    pub mod routing_middleware;
    pub mod dummy_middleware;
    pub mod logging_middleware;
}


extern crate strum;
#[macro_use]
extern crate strum_macros;

fn main() {
    let mut server = http_server::HttpServer::new();
    server.add_route("/hello", |_: &http_request::HttpRequest| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
        .with_body("Hello, World!")
    });

    server.add_route("/", |_: &http_request::HttpRequest| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
    });

    server.add_route("/user-agent", |req: &http_request::HttpRequest| {
        http_response::HttpResponse::new(http_response::HttpStatusCode::OK)
        .with_body(req.headers.get("User-Agent").unwrap_or(&"".to_string()).as_str())
    });
    server.run("127.0.0.1:4221");
}