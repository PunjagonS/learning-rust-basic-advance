/*
    Tower มี middleware แบบจัดการ load, retry, timeout, cookies เหมาะกับ HTTP server
    Hyper เป็น HTTP library มี middleware พวก logging, CORS, compression
*/

#![allow(unused)] // For beginning only.

mod ctx;
mod error;
mod log;
mod model;
mod web;

use ctx::Ctx;
pub use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Model Controller.
    let mc = ModelController::new().await?;
    let mc_clone = mc.clone();

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        /*
            Middleware Layer
            working from bottom to top meaning that other layers
            will have cookie data because cookie layer will be
            executed first from bottom of other layers.
        */
        // .merge(Router::new().route("/hello", get(handler_hello))) // Hello
        .merge(routes_hello()) // Won't be impacted by middleware `mw_require_auth`
        .merge(web::routes_login::routes()) // Won't be impacted by middleware `mw_require_auth`.
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper)) // Map response before return to client.
        // Middleware for cookie extractor, where cookie provided by cookie manager layer below.
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --- Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("--> LISTENING on 127.0.0.1:8080\n");

    axum::serve(listener, routes_all).await.unwrap();
    // endregion: --- Start Server

    Ok(())
}

// middleware
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response.
    let error_response = client_status_error
        .as_ref() // To reuse it later.
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    /*
                        `client_error.as_ref()` allows to directly work
                        with the String inside the Option. not the same
                        with `&client_error` resulting in `&Option<String>`
                        still need to handle the Option to get to the String.
                    */
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("  ->> client_error_body: {client_error_body}");

            // Build the new response from the client error.
            /*
                Since StatusCode derive Copy trait, dereferencing it
                means actually copy it and then after it can take
                the ownership but the status code is still available
                for later.
            */
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1; // Unzip the tuple and get only ClientError.
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
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
