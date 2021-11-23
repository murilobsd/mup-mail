use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(config::File::with_name("config.toml"))
            .unwrap();
        settings
            .merge(config::Environment::with_prefix("MUP_MAIL"))
            .unwrap();

        settings
    });
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIG.read().unwrap().get::<T>(key).unwrap()
}
