/*
    Tower มี middleware แบบจัดการ load, retry, timeout, cookies เหมาะกับ HTTP server
    Hyper เป็น HTTP library มี middleware พวก logging, CORS, compression
*/

#![allow(unused)] // For beginning only.

mod error;
mod web;

pub use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::routes_login;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello()) // Compose multiple routes together
        .merge(routes_login::routes())
        /*
            Middleware Layer
            working from bottom to top meaning that other layers
            will have cookie data because cookie layer will be
            executed first from bottom of other layers.
        */
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING on 127.0.0.1:8080\n");

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: --- Start Server
}

// middleware
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>"))
}
// endregion: --- Routes Hello
