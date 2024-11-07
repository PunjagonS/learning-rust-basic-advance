#![allow(unused)] // For beginning only.

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello()); // Compose multiple routes together

    // region: --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING on 127.0.0.1:8080\n");

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: --- Start Server
}

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Jav@69`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>"))
}
// endregion: --- Routes Hello
