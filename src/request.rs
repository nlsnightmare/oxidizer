use tokio::net::TcpStream;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

use regex::Regex;

use crate::http::{ 
    HttpMethod,
    ContentType
};


pub type HttpParams = HashMap<String, String>;

pub struct Request {
    body: String,
    method: HttpMethod,
    content_type: ContentType,
    path: String,
    params: HttpParams
}

impl Request {
    pub async fn new (stream: &mut TcpStream) -> Option<Request> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.ok()?;

        let body: String = String::from_utf8_lossy(&buffer[..]).to_string(); 
        let  (method, path, params) = Request::parse_head(&body)?;

  

        Some(Request {
            body, method, path,
            content_type: ContentType::Json,
            params
        })
    }

    fn parse_head(body: &str) -> Option<(HttpMethod, String, HttpParams)>{
        let regex = Regex::new(r"^(GET|POST|PUT|PATCH|DELETE) ([^?#]*)+\??(.*) HTTP/\d.\d").ok()?;

        let captures = regex.captures(&body)?;
        let method = match captures.get(1)?.as_str() { 
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("invalid request method")
        };

        let path = captures.get(2)?.as_str().to_string();
        let params = Request::parse_params(&captures.get(3)?.as_str());


        Some((method, path, params))
    }

    fn parse_params(params: &str) -> HttpParams {
        params.split('&').filter_map(|param| {
            let split: Vec<_> = param.split('=').take(2)
                .collect();

            match split.as_slice() {
                &[a, b] => Some((String::from(a), b.to_string())),
                &[a] => Some((String::from(a), "1".to_string())),
                _ => None
            }
        }).collect::<HttpParams>()
    }

    pub fn params(&self) -> &HttpParams {
        &self.params
    }
    pub fn body(&self) -> &String {
        &self.body
    }
    pub fn path(&self) -> &String {
        &self.path
    }
    pub fn method(&self) -> HttpMethod {
        self.method
    }
}

use std::fmt;
impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.method, self.path)
    }
}