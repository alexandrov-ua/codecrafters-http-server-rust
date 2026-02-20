mod http_request;
mod http_response;
mod http_server;
mod some_tests;
mod url_matcher;
mod middlewares;
mod http_context;

use crate::http_context::HttpContext;
use crate::http_request::HttpRequest;
use crate::http_response::{HttpResponse, HttpStatusCode};
use clap::Parser;
use crate::middlewares::{EncodingMiddleware, LoggingMiddleware, PanicMiddleware, StaticFilesMiddleware};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    directory: String,
}

extern crate strum;
#[macro_use]
extern crate strum_macros;

fn main() {
    let mut server = http_server::HttpServer::new();

    server.add_route("/hello", |_: &HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK).with_body("Hello, World!")
    });

    server.add_route("/", |_: &HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK)
    });

    server.add_route(
        "/echo/{message}",
        |_: &HttpRequest, context: &HttpContext| {
            HttpResponse::new(HttpStatusCode::OK).with_body(
                context
                    .get_path_param("message")
                    .unwrap_or(&"".to_string())
                    .as_str(),
            )
        },
    );

    server.add_route("/user-agent", |req: &HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK).with_body(
            req.headers
                .get("User-Agent")
                .unwrap_or(&"".to_string())
                .as_str(),
        )
    });

    server.add_route("/delay", |req: &HttpRequest, _: &HttpContext| {
        let delay_seconds: u64 = req
            .query_params
            .get("sec")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
        HttpResponse::new(HttpStatusCode::OK).with_body("Delayed response")
    });

    server.add_route("/panic", |_: &HttpRequest, _: &HttpContext| {
        panic!("Intentional panic for testing");
    });

    server.add_route("/divide", |req: &HttpRequest, _: &HttpContext| {
        let a = req
            .query_params
            .get("a")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        let b = req
            .query_params
            .get("b")
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(1);
        let res = a / b;
        HttpResponse::new(HttpStatusCode::OK).with_body(&res.to_string())
    });

    server.use_middleware(Box::new(
        PanicMiddleware::new(),
    ));
    server.use_middleware(Box::new(
        LoggingMiddleware::new(),
    ));
    server.use_middleware(Box::new(
        EncodingMiddleware::new(),
    ));

    let args = Args::parse();
    server.use_middleware(Box::new(
        StaticFilesMiddleware::new("/files", &args.directory),
    ));

    server.run("127.0.0.1:4221");
}
