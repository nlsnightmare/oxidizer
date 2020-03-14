use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use serde::Serialize;

use crate::http::{ 
    ContentType, HttpStatus
};

pub struct Response {
    content_type: Option<ContentType>,
    headers: Vec<String>,
    status: HttpStatus,
    http_version: String,
    content: String
}

impl Response {
    pub fn new() -> Response {
        Response {
            content: String::new(),
            content_type: None,
            status: HttpStatus::OK,
            http_version: String::from("1.1"),
            headers: vec!["HTTP/1.1 200 OK".to_string()],
        }
    }

    pub fn json<T>(object: T) -> Response
    where T: Serialize
    {
        let content = serde_json::to_string(&object).unwrap();

        Response {
            content, content_type: Some(ContentType::Json),
            http_version: String::from("1.1"),
            headers: vec!["HTTP/1.1 200 OK".to_string()],
            status: HttpStatus::OK,
        }
    }

    pub fn error() -> Response {
        Response {
            content: String::from("an error has occured"),
            content_type: Some(ContentType::Json),
            http_version: String::from("1.1"),
            headers: vec!["HTTP/1.1 500 Internal Server Error".to_string()],
            status: HttpStatus::OK,
        }
    }

    pub fn content(mut self, data: String, content_type: ContentType) -> Self {
        self.content = data;
        self.content_type = Some(content_type);

        self
    }

    pub async fn send(mut self, mut stream: TcpStream) -> Result<(), String> {
        self.headers.push(self.content_type.unwrap().into());
        let length = format!("Content-Length: {}", self.content.len());
        self.headers.push(length);
        let response = format!("{}\n\n{}", self.headers.join("\n"), self.content);

        println!("---\n{}\n---", response);
        stream.write_all(response.as_bytes()).await.unwrap();
        Ok(())
    }
    
    pub fn not_found() -> Self {
        println!("ready to send 404");
        let mut response = Self::new();
        response.content_type = Some(ContentType::Text);
        response.headers[0] = "HTTP/1.1 404 Not Found".to_string();
        response.content = String::from("<h1>404 - Not found<h1>");
        response
    }

    pub fn redirect(target: &str) -> Self {
        let mut response = Self::new();
        response.content_type = Some(ContentType::Text);
        response.headers.push("HTTP/1.1 301 Moved Permanently".to_string());
        response.headers.push(format!("Location: {}", target));
        response
    }
}

