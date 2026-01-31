use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum HttpStatusCode {
    OK = 200,
    Created = 201,
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
            HttpStatusCode::Created => write!(f, "Created"),
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
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(status_code: HttpStatusCode) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "0".to_string());
        HttpResponse {
            status_code,
            headers,
            body: None,
        }
    }

    pub fn status_code(&self) -> HttpStatusCode {
        self.status_code
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.set_header(key, value);
        self
    }

    pub fn with_body(self, body: &str) -> Self {
        self.with_bytes_body(body.as_bytes().to_vec(), "text/plain")
    }

    pub fn get_body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    pub fn with_bytes_body(mut self, body: Vec<u8>, content_type: &str) -> Self {
        self.set_header("Content-Length", body.len().to_string().as_str());
        self.set_header("Content-Type", content_type);
        self.body = Some(body);
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response_str = format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, self.status_code.to_string());
        for (key, value) in &self.headers {
            response_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        response_str.push_str("\r\n");
        let mut response = response_str.as_bytes().to_vec();
        if let Some(body) = &self.body {
            response.extend_from_slice(body);
        }
        response
    }
}

impl From<HttpResponse> for Vec<u8> {
    fn from(response: HttpResponse) -> Self {
        response.to_bytes()
    }
}

