#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET, POST, PUT, DELETE, PATCH
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    File, Json, Text, Empty, Html
}

impl Into<String> for ContentType {
    fn into(self) -> String {

        let result = match self {
            ContentType::Html => "text/html",
            ContentType::Text => "text/html",
            ContentType::File => "text/html",
            ContentType::Json => "application/json",
            _ => panic!("invalid Content-Type: {:?}", self)
        };


        format!("Content-Type: {}", result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpError {
    OK,
    NotFound,
    InternalServerError,
    Forbidden,
    Unauthorized,
}

pub type HttpStatus = HttpError;

impl Into<String> for HttpError {
    fn into(self) -> String {
        let result = match self {
            HttpError::OK => "200 Ok",
            HttpError::Unauthorized => "401 Unauthorized",
            HttpError::Forbidden => "403 Forbidden",
            HttpError::NotFound => "404 Not Found",
            HttpError::InternalServerError => "500 Internal Server Error",
        };

        String::from(result)
    }
}


impl From<serde_json::Error> for HttpError {
    fn from(_error: serde_json::Error) -> Self {
        HttpError::InternalServerError
    }
}

use std::fmt;
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}