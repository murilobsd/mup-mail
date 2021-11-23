use anyhow::Result;
use async_trait::async_trait;

// use super::error::ServiceError;
use crate::application::port::incoming::send_activate_use_case::SendActivateUseCase;

/// The NewUserService implements the [NewUserCase]().
#[allow(dead_code)]
pub struct SendActivateAccountService {}

impl SendActivateAccountService {
    /// Create NewUserService
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SendActivateUseCase for SendActivateAccountService {
    async fn send(&self, _email: &str) -> Result<()> {
        todo!()
    }
}
