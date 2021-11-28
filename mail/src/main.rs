mod config;

use std::sync::Arc;

use deadpool_lapin::Runtime;

use mail_application::application::service::send_activate_service::SendActivateAccountService;
use provider::mailgun_adapter::MailGunAdapter;
#[allow(unused_imports)]
use tokio_amqp::LapinTokioExt as _;

use publish::user_publisher_adapter::{PublisherState, UserPublisherAdapter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let amqp_addr: String = config::get("amqp_addr");
    let cfg = deadpool_lapin::Config {
        url: Some(amqp_addr),
        ..Default::default()
    };

    // service
    let send_mail_port = MailGunAdapter::default();
    let send_activate_service =
        SendActivateAccountService::new(Box::new(send_mail_port));

    // state
    let publish_state = PublisherState::new(Arc::new(send_activate_service));

    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    let publisher_user = UserPublisherAdapter::new(publish_state, pool);

    publisher_user.run().await.unwrap();

    Ok(())
}
