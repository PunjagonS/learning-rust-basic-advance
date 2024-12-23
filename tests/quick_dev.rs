#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/hello?name=Jav@69").await?.print().await?;
    hc.do_get("/hello2/Jiw2").await?.print().await?;

    // hc.do_get("/src/bin/axum/hello_router.rs")
    //     .await?
    //     .print()
    //     .await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome",
        }),
    );
    // req_login.await?.print().await?;

    // hc.do_get("/hello2/Jiw2").await?.print().await?; // Test cookie is stored on client

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA",
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;
    // hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
