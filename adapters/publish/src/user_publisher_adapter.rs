use anyhow::Result;
use async_trait::async_trait;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties,
};
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

use user_application::application::port::outgoing::new_user_publish_port::NewUserPublishPort;

#[derive(Default, Debug, Clone)]
struct NewUserQueue {}

impl NewUserQueue {
    async fn open(&self) -> Result<Channel> {
        let con = Connection::connect(
            "amqp://localhost:5672/%2f",
            ConnectionProperties::default(),
        )
        .await?;
        let ch = con.create_channel().await?;
        ch.queue_declare(
            "new_user",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

        Ok(ch)
    }

    pub async fn publish(&self, msg: Vec<u8>) -> Result<()> {
        let ch = self.open().await?;
        info!("publish task {}://{}", "new_user", "1");
        ch.basic_publish(
            "",
            "new_user",
            BasicPublishOptions::default(),
            msg,
            BasicProperties::default()
                .with_message_id("1".into())
                .with_content_type("application/text".into())
                .with_timestamp(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("get timestamp")
                        .as_secs(),
                ),
        )
        .await?
        .await?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct UserPublisherAdapter {
    new_user: NewUserQueue,
}

impl UserPublisherAdapter {
    pub fn new() -> Self {
        Self {
            new_user: NewUserQueue::default(),
        }
    }
}

#[async_trait]
impl NewUserPublishPort for UserPublisherAdapter {
    async fn new_user_publish(&self, _email: &str) -> Result<()> {
        let msg: Vec<u8> = String::from("ola mundo").into_bytes();
        self.new_user.publish(msg).await?;

        Ok(())
    }
}
