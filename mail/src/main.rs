mod config;

use std::sync::Arc;

use deadpool_lapin::Runtime;

#[allow(unused_imports)]
use tokio_amqp::LapinTokioExt as _;

use publish::user_publisher_adapter::UserPublisherAdapter;
use provider::mailgun_adapter::MailGunAdapter;
use mail_application::application::port::incoming::send_activate_use_case::SendActivateUseCase;
use mail_application::application::service::new_user_service::SendActivateAccountService;

struct MailState {
    activate_acount: Arc<dyn SendActivateUseCase + Send + Sync>
}

impl MailState {
    pub fn new( activate_acount: Arc<dyn SendActivateUseCase + Send + Sync>) -> Self {
        Self { activate_acount }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let amqp_addr: String = config::get("amqp_addr");
    let cfg = deadpool_lapin::Config {
        url: Some(amqp_addr),
        ..Default::default()
    };

    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    let publisher_user = UserPublisherAdapter::new(pool);

    let mailgun_adapter = MailGunAdapter::default();

    let activate_acount_service = SendActivateAccountService::new(Box::new(mailgun_adapter));

    let _mail_state = MailState::new(Arc::new(activate_acount_service));

    publisher_user.run().await.unwrap();

    Ok(())
}
