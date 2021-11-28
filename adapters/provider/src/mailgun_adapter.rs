use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use mail_application::application::port::outgoing::send_mail_port::SendMailPort;
use reqwest::Error;
use serde::de::DeserializeOwned;

#[derive(Default, Debug, Clone)]
pub struct MailGunAdapter {}

/// Params to send a email
pub struct EmailParams {
    /// Sender
    pub from: String,
    /// Receiver
    pub to: String,
    /// Title of the email
    pub subject: String,
    /// Text body format of the email. If text is set html should be `None`
    pub text: Option<String>,
    /// Html body format of the email. If html is set text should be `None`
    pub html: Option<String>,
}

pub async fn send_email<T>(email_params: EmailParams) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut form = HashMap::new();
    form.insert("from", email_params.from);
    form.insert("to", email_params.to);
    form.insert("subject", email_params.subject);

    if let Some(html_message) = email_params.html {
        form.insert("html", html_message);
    } else if let Some(text_message) = email_params.text {
        form.insert("text", text_message);
    }

    let response = reqwest::Client::new()
        .post("https://api:@/v3//messages")
        .form(&form)
        .send()
        .await?
        .json::<T>()
        .await;

    response
}

#[async_trait]
impl SendMailPort for MailGunAdapter {
    async fn send(&self, email: &str) -> Result<()> {
        let params = EmailParams {
            from: "Excited User <mailgun@mg.microbio.rs>".to_string(),
            to: email.to_string(),
            subject: "test mailgung api".to_string(),
            text: Some("hello this is a test".to_string()),
            html: None,
        };

        // let mut mailgun =
        //     MailgunApi::new("", "api.mailgun.net", "");
        // let response =
        //     mailgun.send_email::<HashMap<String, String>>(params).await;

        send_email(params).await?;
        println!("eita");

        Ok(())
    }
}
