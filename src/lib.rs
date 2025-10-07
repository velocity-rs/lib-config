use config::{Config, ConfigError};
use log::{debug, error, info};
use std::{fmt::Debug, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get<'a, T>(key: &str) -> Option<T>
where
    T: serde::Deserialize<'a> + Debug,
{
    if CONFIG.get().is_none() {
        debug!("Loading configurations....");
        let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("VELOCITY").prefix_separator("_"))
            .build();

        match config {
            Ok(result) => {
                info!("Configurations loaded successfully");
                let _ = CONFIG.set(result);
            }
            Err(e) => panic!("Config error {}", e),
        }
    }

    let config = CONFIG.get().unwrap();
    //TODO: CHECK CASE SENSITIVITY
    match config.get::<T>(&key) {
        Ok(val) => Some(val),
        Err(e) => match e {
            ConfigError::NotFound(_) => {
                return None;
            }
            _ => {
                error!("Error getting config value for key {}: {}", key, e);
                return None;
            }
        },
    }
}
