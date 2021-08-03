use anyhow::*;
use hyper::Server;
use log::info;
use routerify::RouterService;
use std::net::SocketAddr;

use facade::*;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging_service()?;
    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a Service from the router above to handle incoming requests.
    let router = router();
    let service = RouterService::new(router).unwrap();

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    info!("App is running on: {}", addr);
    info!("Try calling http://localhost:3000/uuid to test the proxy.");
    server
        .await
        .context("Fatal server error resulting in the hyper server stopping")
}
