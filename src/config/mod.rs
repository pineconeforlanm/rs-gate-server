mod database;
mod server;

use config::Config;
use serde::Deserialize;
use std::sync::LazyLock;

use crate::app::ApiResult;
pub use database::DatabaseConfig;
pub use server::ServerConfig;

static CONFIG: LazyLock<AppConfig> =
  LazyLock::new(|| AppConfig::load().expect("Failed to initialize config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
  server: ServerConfig,
  database: DatabaseConfig,
}

impl AppConfig {
  pub fn load() -> ApiResult<Self> {
    let config = Config::builder()
      .add_source(
        config::File::with_name("config/application")
          .format(config::FileFormat::Yaml)
          .required(true),
      )
      .add_source(
        config::Environment::with_prefix("APP")
          .try_parsing(true)
          .separator("_")
          .list_separator(","),
      )
      .build()?;
    Ok(config.try_deserialize()?)
  }

  pub fn server(&self) -> &ServerConfig {
    &self.server
  }

  pub fn database(&self) -> &DatabaseConfig {
    &self.database
  }
}

pub fn get() -> &'static AppConfig {
  &CONFIG
}
