use anyhow::Result;
use deadpool_lapin::Pool;
use futures_util::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::info;
use std::time::Duration;

#[derive(Debug, Clone)]
struct NewUserQueue {
    pool: Pool,
}

impl NewUserQueue {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn consume(&self) -> Result<()> {
        let mut retry_interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            retry_interval.tick().await;

            let rmq_con = self.pool.get().await?;
            let channel = rmq_con.create_channel().await?;

            let queue = channel
                .queue_declare(
                    "new_user",
                    QueueDeclareOptions::default(),
                    FieldTable::default(),
                )
                .await?;
            info!("Declared queue {:?}", queue);

            let mut consumer = channel
                .basic_consume(
                    "new_user",
                    "my_consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await?;

            info!("rmq consumer connected, waiting for messages");

            while let Some(delivery) = consumer.next().await {
                if let Ok((channel, delivery)) = delivery {
                    info!("received msg: {:?}", delivery);
                    channel
                        .basic_ack(
                            delivery.delivery_tag,
                            BasicAckOptions::default(),
                        )
                        .await?
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserPublisherAdapter {
    new_user: NewUserQueue,
}

impl UserPublisherAdapter {
    pub fn new(pool: Pool) -> Self {
        Self {
            new_user: NewUserQueue::new(pool),
        }
    }

    pub async fn run(&self) -> Result<()> {
        self.new_user.consume().await?;
        Ok(())
    }
}
