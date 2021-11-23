mod config;

use std::sync::Arc;

use publish::user_publisher_adapter::UserPublisherAdapter;

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database_url: String = config::get("database_url");

    Ok(())
}
