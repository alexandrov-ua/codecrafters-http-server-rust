use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum HttpStatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatusCode::OK => write!(f, "OK"),
            HttpStatusCode::BadRequest => write!(f, "Bad Request"),
            HttpStatusCode::NotFound => write!(f, "Not Found"),
            HttpStatusCode::InternalServerError => write!(f, "Internal Server Error"),
            HttpStatusCode::NotImplemented => write!(f, "Not Implemented"),
            HttpStatusCode::ServiceUnavailable => write!(f, "Service Unavailable"),
        }
    }
}
    

pub struct HttpResponse {
    status_code: HttpStatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: HttpStatusCode) -> Self {
        HttpResponse {
            status_code,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn status_code(&self) -> HttpStatusCode {
        self.status_code
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.set_header(key, value);
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.set_header("Content-Length".to_string(), body.len().to_string());
        self.set_header("Content-Type".to_string(), "text/plain".to_string());
        self.body = Some(body.to_string());
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, self.status_code.to_string());
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        response.into_bytes()
    }
}

impl From<HttpResponse> for Vec<u8> {
    fn from(response: HttpResponse) -> Self {
        response.to_bytes()
    }
}

