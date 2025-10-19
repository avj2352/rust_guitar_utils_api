#![allow(unused)]

use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;

#[cfg(test)]

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() -> Result<()> {
        let hc = new_client("http://localhost:8080")?;
        hc.do_get("/health").await?.print().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_greet_endpoint() -> Result<()> {
        let hc = new_client("http://localhost:8080")?;
        hc.do_get("/greet/pramod").await?.print().await?;
        Ok(())
    }
}
