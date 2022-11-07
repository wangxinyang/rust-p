use std::net::SocketAddr;

use axum::Server;
use axum::{routing::get, Router};
use axumr::ws_handler;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8200));
    let app = Router::new().route("/ws", get(ws_handler));

    println!("Server running on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
