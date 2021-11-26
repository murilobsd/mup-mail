use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use mailgun_api::api::EmailParams;
use mailgun_api::MailgunApi;

use mail_application::application::port::outgoing::send_mail_port::SendMailPort;

#[derive(Default, Debug, Clone)]
pub struct MailGunAdapter {}

#[async_trait]
impl SendMailPort for MailGunAdapter {
    async fn send(&self, _email: &str) -> Result<()> {
        let params = EmailParams {
            from: "Excited User <mailgun@mg.microbio.rs>".to_string(),
            to: "".to_string(),
            subject: "test mailgung api".to_string(),
            text: Some("hello this is a test".to_string()),
            html: None,
        };

        let mut mailgun = MailgunApi::new(
            "",
            "api.mailgun.net",
            "",
        );
        let response =
            mailgun.send_email::<HashMap<String, String>>(params).await;

        assert!(response.is_ok());

        Ok(())
    }
}
