use anyhow::Result;
use async_trait::async_trait;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection,
    ConnectionProperties,
};
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Debug, Clone)]
struct NewUserQueue {}

impl NewUserQueue {
    async fn open(&self) -> Result<Channel> {
        todo!()
    }

    pub async fn publish(&self, msg: Vec<u8>) -> Result<()> {
        todo!()
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
