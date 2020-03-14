mod macros;

use regex::Regex;

use crate::request::Request;
use crate::response::Response;
use crate::http::HttpError;
use crate::http::HttpMethod;

type Handler = dyn Fn(Request) -> Result<Response, HttpError>;
pub struct Route {
    url: Regex,
    handler: Box<Handler>,
    method: HttpMethod,
    action: String
}

impl Route {
    fn new<F>(method: HttpMethod, url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {

        let named_regex = Regex::new(r":([a-z]+)")
            .expect("named regex failed to compile!");

        let url_params: Vec<_> = named_regex.captures_iter(url)
            .filter_map(|capture| capture.get(1))
            .map(|m| m.as_str())
            .collect();

        let mut url = String::from(url);
        for param in &url_params {
            let a = format!(":{}", &param);
            url = url.replace(&a, "(.*)");
        }

        Self {
            url: Regex::new(&url).unwrap(),
            handler: Box::new(handler),
            method, action
        }
    }

    pub fn get<F>(url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {
        Self::new::<F>(HttpMethod::GET, url, action, handler)
    }
    pub fn post<F>(url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {
        Self::new::<F>(HttpMethod::POST, url, action, handler)
    }
    pub fn put<F>(url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {
        Self::new::<F>(HttpMethod::PUT, url, action, handler)
    }
    pub fn patch<F>(url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {
        Self::new::<F>(HttpMethod::PATCH, url, action, handler)
    }
    pub fn delete<F>(url: &str, action: String, handler: F) -> Self 
    where F: 'static + Fn(Request) -> Result<Response, HttpError> {
        Self::new::<F>(HttpMethod::DELETE, url, action, handler)
    }


    pub fn matches(&self, url: &str) -> bool {
        self.url.captures(url).is_some()
    }

    pub fn handle(&self, request: Request) -> Result<Response, HttpError> {
        let handler = &self.handler;
        handler(request)
    }
}



use std::fmt;
impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} /{} {}", self.method, self.url, self.action)
    }
}