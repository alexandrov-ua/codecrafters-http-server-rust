use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;


pub struct EncodingMiddleware;

impl EncodingMiddleware {
    pub fn new() -> Self {
        EncodingMiddleware
    }

    pub fn encode(&self, data: &str) -> Vec<u8> {
        // Example encoding logic (UTF-8)
        data.as_bytes().to_vec()
    }

    pub fn decode(&self, data: &[u8]) -> String {
        // Example decoding logic (UTF-8)
        String::from_utf8_lossy(data).to_string()
    }
}

impl HttpMiddleware for EncodingMiddleware {
    fn handle(
        &self,
        request: &mut HttpRequest,
        next: &dyn Fn(&mut HttpRequest) -> HttpResponse,
    ) -> HttpResponse {
        let mut is_gzip: bool = false;
        if let Some(encoding) = request.headers.get("Accept-Encoding") {
            is_gzip = encoding.split(',').map(|s| s.trim()).any(|s| s == "gzip");
        }

        let response = next(request);

        if is_gzip {
            request
                .headers
                .insert("Content-Encoding".to_string(), "gzip".to_string());

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            if let Some(body) = response.get_body() {
                encoder.write_all(body).unwrap();
            }
            let compressed_body = encoder.finish().unwrap();

            let content_type = response
                .get_header("Content-Type")
                .map(|s| s.to_string())
                .unwrap_or("application/octet-stream".to_string());

            return response
                .with_bytes_body(compressed_body, &content_type)
                .with_header("Content-Encoding", "gzip");
        }

        response
    }
}
