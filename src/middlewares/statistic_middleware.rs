use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::middlewares::http_middleware::HttpMiddleware;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct StatisticMiddleware {
    path: String,
    statistic: Mutex<Statistic>,
}

struct Statistic{
    total_requests: u64,
    requests_by_path: HashMap<String, u64>,
    response_statuses: HashMap<u16, u64>,
}

impl StatisticMiddleware {
    pub fn new(path: &str) -> Self {
        StatisticMiddleware {
            path: path.to_string(),
            statistic: Mutex::new(Statistic {
                total_requests: 0,
                requests_by_path: HashMap::new(),
                response_statuses: HashMap::new(),
            }),
        }
    }
}

impl HttpMiddleware for StatisticMiddleware {
    fn handle(
        &self,
        request: &mut HttpRequest,
        next: &dyn Fn(&mut HttpRequest) -> HttpResponse,
    ) -> HttpResponse {
        if request.path == self.path {
            let statistic = self.statistic.lock().unwrap();
            let body = format!(
                "Total Requests: {}\nRequests by Path: {:?}\nResponse Statuses: {:?}",
                statistic.total_requests, statistic.requests_by_path, statistic.response_statuses
            );
            HttpResponse::new(crate::http_response::HttpStatusCode::OK).with_body(&body)
        } else {
            let mut statistic = self.statistic.lock().unwrap();

            statistic.total_requests += 1;

            *statistic.requests_by_path
                .entry(format!("{} {}", request.method, request.path))
                .or_insert(0) += 1;
            let response = next(request);
            *statistic.response_statuses
                .entry(response.status_code() as u16)
                .or_insert(0) += 1;
            return response;
        }
    }
}
