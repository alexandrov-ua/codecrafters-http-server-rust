use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader, Result};
use std::str::FromStr;

#[derive(Debug, Display, Clone, Copy)]
pub enum HttpStatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
}
    

pub struct HttpResponse {
    status_code: HttpStatusCode,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn new(status_code: HttpStatusCode) -> Self {
        HttpResponse {
            status_code,
            headers: HashMap::new(),
        }
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code as u16, self.status_code);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.into_bytes()
    }
}