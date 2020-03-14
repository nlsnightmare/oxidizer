use oxidizer::server::Server;

#[tokio::main]
async fn main() {

    use oxidizer::usercontroller::index;
    let mut server = Server::new();
    let routes = vec![
        oxidizer::get!("users", index),
    ];

    // server.routes(routes);

    server.listen().await;
}