use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader, Result};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(EnumString, Debug, PartialEq, Display)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}

pub struct HttpRequest<'a>{
    pub method: HttpMethod,
    pub path: String,
    pub qury: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub content: Box<HttpRequestContent<BufReader<& 'a mut dyn Read>>>,
    pub query_params: HashMap<String, String>,
}

use std::cell::Cell;


pub struct HttpRequestContent<T: BufRead> {
    body: Cell<T>
}

impl<T: BufRead> HttpRequestContent<T> {
    pub fn to_string(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.body.get_mut().read_to_string(&mut buf)?;
        Ok(buf)
    }
}

impl<'a> HttpRequest<'a> {
    pub fn from_reader(r: &'a mut dyn Read) -> Result<HttpRequest> {
        let mut buf = BufReader::new(r);
        let mut first_line = String::new();
        let _ = buf.read_line(&mut first_line)?;
        first_line = String::from(first_line.trim_end());
        let (method, path, query, http_version, query_params) = process_start_line(first_line)?;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut line = String::new();
        let mut read = buf.read_line(&mut line)?;
        while read > 0 && line.trim_end() != "" {
            let (n, v) = line.split_once(":").unwrap_or((&line, ""));
            headers.insert(String::from(n), String::from(v.trim()));
            line = String::new();
            read = buf.read_line(&mut line)?;
        }
        Ok(HttpRequest {
            method: method,
            path: path,
            qury: query,
            http_version: http_version,
            headers: headers,
            content: Box::new(HttpRequestContent {
                body: Cell::new(buf),
            }),
            query_params: query_params,
        })
    }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        map.insert(String::from(k), String::from(v));
    }
    map
}

fn process_start_line(s: String) -> Result<(HttpMethod, String, String, String, HashMap<String, String>)> {
    let mut parts = s.split(' ');
    let method = HttpMethod::from_str(parts.next().unwrap()).unwrap();
    let path_and_query = parts.next().unwrap();
    let path_and_query_split = path_and_query
        .split_once("?")
        .unwrap_or((path_and_query, ""));
    let path = path_and_query_split.0;
    let query = path_and_query_split.1;
    let query_params = parse_query_string(query);
    let http_ver = parts.next().unwrap();
    return Ok((
        method,
        String::from(path),
        String::from(query),
        String::from(http_ver),
        query_params,
    ));
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn from_reader_smoke() {
        let requessst_str = "GET /qwe?p=1 HTTP/1.1\r\nHost: 127.0.0.1:4221\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n";
        let mut reader = Cursor::new(requessst_str.as_bytes());
        let request = HttpRequest::from_reader(&mut reader).unwrap();
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/qwe");
        assert_eq!(request.qury, "p=1");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.headers["Host"], "127.0.0.1:4221");
        assert_eq!(request.headers["User-Agent"], "curl/8.5.0");
    }

    #[test]
    fn test_content_to_string() {
        let requessst_str = "POST /api/user HTTP/1.1\r\nHost: 127.0.0.1:4221\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\nContent-Length: 22\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nname=admin&shoesize=12";
        let mut reader = Cursor::new(requessst_str.as_bytes());
        let request = HttpRequest::from_reader(&mut reader).unwrap();
        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path, "/api/user");
        assert_eq!(request.qury, "");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.headers["Host"], "127.0.0.1:4221");
        assert_eq!(request.headers["User-Agent"], "curl/8.5.0");
        assert_eq!(request.headers["Content-Length"], "22");
        assert_eq!(request.headers["Content-Type"], "application/x-www-form-urlencoded");
        let mut content = request.content;
        let content_str = content.to_string().unwrap();
        assert_eq!(content_str, "name=admin&shoesize=12");
    }

    #[test]
    fn from_reader_qury_params() {
        let requessst_str = "GET /qwe?p=1&p1=wer&q HTTP/1.1\r\nHost: 127.0.0.1:4221\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n";
        let mut reader = Cursor::new(requessst_str.as_bytes());
        let request = HttpRequest::from_reader(&mut reader).unwrap();
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/qwe");
        assert_eq!(request.qury, "p=1&p1=wer&q");
        assert_eq!(request.query_params["p"], "1");
        assert_eq!(request.query_params["p1"], "wer");
        assert_eq!(request.query_params["q"], "");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.headers["Host"], "127.0.0.1:4221");
        assert_eq!(request.headers["User-Agent"], "curl/8.5.0");
    }
}