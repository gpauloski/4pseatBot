use std::path::Path;
use anyhow::Result;

use ::config::{Config,Environment,File,FileFormat};
use dotenv::dotenv;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: String,
}


impl BotConfig {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        if Path::new(".env").exists() {
            dotenv()?;
        }

        let mut builder = Config::builder().add_source(
            Environment::with_prefix("BOT").try_parsing(true).separator("_")
        );
   
        if let Some(path) = config_path {
            builder = builder.add_source(File::new(path, FileFormat::Toml));
        }

        let config: BotConfig = builder.build()?.try_deserialize()?;
        Ok(config)
    }
}
