use anyhow::Result;
use deadpool_lapin::Pool;
use futures_util::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use log::info;
use mail_application::application::port::incoming::send_activate_use_case::SendActivateUseCase;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
struct NewUserQueue {
    pool: Pool,
}

impl NewUserQueue {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn consume(&self, state: PublisherState) -> Result<()> {
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
                    let email =
                        match String::from_utf8(delivery.data.to_owned()) {
                            Ok(v) => v,
                            Err(e) => panic!("invalid utf8 {}", e),
                        };
                    info!("received msg: {:?}", &email);
                    state.send_activate_service.send(&email).await?;
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

#[derive(Clone)]
pub struct PublisherState {
    send_activate_service: Arc<dyn SendActivateUseCase + Send + Sync>,
}

impl PublisherState {
    pub fn new(
        send_activate_service: Arc<dyn SendActivateUseCase + Send + Sync>,
    ) -> Self {
        Self {
            send_activate_service,
        }
    }
}

// TODO: fix remove state here or change to another struct
#[derive(Clone)]
pub struct UserPublisherAdapter {
    new_user: NewUserQueue,
    state: PublisherState,
}

impl UserPublisherAdapter {
    pub fn new(state: PublisherState, pool: Pool) -> Self {
        Self {
            state,
            new_user: NewUserQueue::new(pool),
        }
    }

    pub async fn run(&self) -> Result<()> {
        self.new_user.consume(self.state.clone()).await?;
        Ok(())
    }
}
