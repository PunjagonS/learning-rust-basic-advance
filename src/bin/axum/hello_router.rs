#![allow(unused)] // For beginning only.

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, <strong>Axum!</strong>") }),
    );

    // region: --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING on 127.0.0.1:8080\n");

    axum::serve(listener, routes_hello).await.unwrap();
    // endregion: --- Start Server
}
