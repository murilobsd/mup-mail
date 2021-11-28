use anyhow::Result;
use async_trait::async_trait;

// use super::error::ServiceError;
use crate::application::port::incoming::send_activate_use_case::SendActivateUseCase;
use crate::application::port::outgoing::send_mail_port::SendMailPort;

#[allow(dead_code)]
pub struct SendActivateAccountService {
    send_mail_port: Box<dyn SendMailPort + Send + Sync>,
}

impl SendActivateAccountService {
    /// Create NewUserService
    pub fn new(send_mail_port: Box<dyn SendMailPort + Send + Sync>) -> Self {
        Self { send_mail_port }
    }
}

#[async_trait]
impl SendActivateUseCase for SendActivateAccountService {
    async fn send(&self, email: &str) -> Result<()> {
        self.send_mail_port.send(email).await
    }
}
