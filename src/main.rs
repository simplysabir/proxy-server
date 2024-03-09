use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::net::SocketAddr;
use tower::make::Shared;

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API Path: {}", path);
    } else {
        println!("Generic Path: {}", path);
    }

    handle(req).await
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(log));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
