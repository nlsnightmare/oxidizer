use tokio::net::{ 
    TcpListener,
    TcpStream
};
use tokio::stream::StreamExt;


use crate::request::Request;
use crate::response::Response;
use crate::router::Route;
use crate::http::{
    HttpMethod,
    HttpError
};

pub struct Server
{
    routes: Vec<Route>,
    port: i32,
    host: String
}

impl Server {
    pub fn new() -> Self {
        dotenv::dotenv().expect("unable to find .env file!");

        let port = dotenv::var("PORT")
            .map_or(8080, |port: String| {
                port.parse::<i32>().expect("port must be a valid integer!") 
            });
        let host = dotenv::var("HOST")
            .unwrap_or(String::from("localhost"));


        Self {
            routes: Vec::new(),
            port, host
        }
    }

    pub async fn listen(self) {
        let host = format!("{}:{}", self.host, self.port);
        let mut listener = TcpListener::bind(host.clone()).await
            .expect(&format!("unable to bind to port {}", self.port));


        println!("oxidizer started in {}", host);
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(socket) => self.handle_connection(socket).await,
                Err(e) => eprintln!("Oxidizer Error: {}", e)
            }
        }

        println!("socket closed");
    }

    async fn handle_connection(&self, mut stream: TcpStream) {
        let request = Request::new(&mut stream).await
            .unwrap();

        println!("{:?}", request);

        let matching_route = self.routes.iter()
            .find(|route| route.matches(request.path()));

        let response = match matching_route {
            Some(route) => route.handle(request),
            None =>  {
                println!("404");
                Ok(Response::not_found())
            }
        };

        response.unwrap_or(Response::error())
            .send(stream).await
            .unwrap();
    }

    pub fn routes(&mut self, route_groups: Vec<Vec<Route>>) {
        for group in route_groups {
            for route in group {
                self.routes.push(route);
            }
        }
    }
}