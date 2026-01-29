use crate::http_request::{HttpRequest, HttpMethod};
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;
use crate::url_matcher::UrlMatcher;

pub struct StaticFilesMiddleware {
    base_path: String,
    matcher: UrlMatcher,
}

impl StaticFilesMiddleware {
    pub fn new(base_url: &str, base_path: &str) -> Self {
        let pattern = format!("{}/{{file_path*}}", base_url);
        StaticFilesMiddleware {
            base_path: base_path.to_string(),
            matcher: UrlMatcher::new(&pattern),
        }
    }
}

impl HttpMiddleware for StaticFilesMiddleware {
    fn handle(
        &self,
        request: &mut HttpRequest,
        next: &dyn Fn(&mut HttpRequest) -> HttpResponse,
    ) -> HttpResponse {
        let (is_matched, params) = self.matcher.match_url(&request.path);
        if !is_matched {
            return next(request);
        }
        let file_path = format!(
            "{}/{}",
            self.base_path,
            params.get("file_path").unwrap_or(&"".to_string())
        );
        match request.method {
            HttpMethod::GET => {
                if let Ok(contents) = std::fs::read_to_string(&file_path) {
                    HttpResponse::new(crate::http_response::HttpStatusCode::OK).with_body(&contents)
                    .with_header("Content-Type", "application/octet-stream")
                } else {
                    next(request)
                }
            }
            HttpMethod::POST => {
                std::fs::write(&file_path, &request.content.to_string().unwrap()).unwrap_or(());
                HttpResponse::new(crate::http_response::HttpStatusCode::Created)
            }
            _ => return next(request),
        }
    }
}
