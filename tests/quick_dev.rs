#![allow(unused)] // For beginning only.

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/hello?name=Jav@69").await?.print().await?;
    hc.do_get("/hello2/Jiw2").await?.print().await?;
    Ok(())
}
