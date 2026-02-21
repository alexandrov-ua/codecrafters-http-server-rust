mod http_context;
mod http_request;
mod http_response;
mod http_server;
mod middlewares;
mod url_matcher;

use crate::http_context::HttpContext;
use crate::http_request::{HttpRequest};
use crate::http_response::{HttpResponse, HttpStatusCode};
use crate::middlewares::{
    EncodingMiddleware, LoggingMiddleware, PanicMiddleware, StaticFilesMiddleware, StatisticMiddleware,
};
use clap::Parser;

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

    server.get("/hello", |_: &mut HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK).with_body("Hello, World!")
    });

    server.get("/", |_: &mut HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK)
    });

    server.get(
        "/echo/{message}",
        |_: &mut HttpRequest, context: &HttpContext| {
            HttpResponse::new(HttpStatusCode::OK).with_body(
                context
                    .get_path_param("message")
                    .unwrap_or(&"".to_string())
                    .as_str(),
            )
        },
    );

    server.get("/user-agent", |req: &mut HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK).with_body(
            req.headers
                .get("User-Agent")
                .unwrap_or(&"".to_string())
                .as_str(),
        )
    });

    server.get("/delay", |req: &mut HttpRequest, _: &HttpContext| {
        let delay_seconds: u64 = req
            .query_params
            .get("sec")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        std::thread::sleep(std::time::Duration::from_secs(delay_seconds));
        HttpResponse::new(HttpStatusCode::OK).with_body("Delayed response")
    });

    server.get("/panic", |_: &mut HttpRequest, _: &HttpContext| {
        panic!("Intentional panic for testing");
    });

    server.get("/divide", |req: &mut HttpRequest, _: &HttpContext| {
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

    server.post("/echo-body", |req: &mut HttpRequest, _: &HttpContext| {
        HttpResponse::new(HttpStatusCode::OK)
            .with_body(req.content.to_string().unwrap_or("".to_string()).as_str())
    });

    server.use_middleware(Box::new(StatisticMiddleware::new("/stats")));
    server.use_middleware(Box::new(PanicMiddleware::new()));
    server.use_middleware(Box::new(LoggingMiddleware::new()));
    server.use_middleware(Box::new(EncodingMiddleware::new()));

    let args = Args::parse();
    server.use_middleware(Box::new(StaticFilesMiddleware::new(
        "/files",
        &args.directory,
    )));

    server.run("127.0.0.1:4221");
}
