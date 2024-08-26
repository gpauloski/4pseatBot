use ::config::{Config, Environment, File, FileFormat};
use derive_more::{Display, From};
use dotenv::dotenv;
use log::info;
use serde::Deserialize;
use std::fmt;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[from]
    Config(config::ConfigError),

    #[from]
    Env(dotenv::Error),
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub guild: u64,
    pub debug: bool,
}

impl BotConfig {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        if Path::new(".env").exists() {
            dotenv()?;
            info!("loaded .env file");
        }

        let mut builder = Config::builder().set_default("debug", false)?.add_source(
            Environment::with_prefix("BOT")
                .try_parsing(true)
                .separator("_"),
        );

        if let Some(path) = config_path {
            builder = builder.add_source(File::new(path, FileFormat::Toml));
        }

        let config: BotConfig = builder.build()?.try_deserialize()?;
        Ok(config)
    }
}

impl fmt::Display for BotConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "config -- token: ..., debug: {}", self.debug)
    }
}
