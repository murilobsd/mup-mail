use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SendActivateUseCase {
    async fn send(&self, email: &str) -> Result<()>;
}
