use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SendMailPort {
    async fn send(&self, email: &str) -> Result<()>;
}
