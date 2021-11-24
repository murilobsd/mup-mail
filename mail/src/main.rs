mod config;

use deadpool_lapin::Runtime;

#[allow(unused_imports)]
use tokio_amqp::LapinTokioExt as _;

use publish::user_publisher_adapter::UserPublisherAdapter;

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

    publisher_user.run().await.unwrap();

    Ok(())
}
