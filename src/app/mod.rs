use crate::config;
use axum::Router;
pub use errors::ApiResult;
pub mod auth;
pub mod common;
mod database;
pub mod enumeration;
pub mod errors;
pub mod id;
mod json;
mod latency;
pub mod logger;
pub mod middleware;
pub mod path;
mod query;
pub mod response;
mod serde;
mod server;
pub mod utils;
pub mod valid;
pub mod validation;

use sea_orm::prelude::*;

#[derive(Clone)]
pub struct AppState {
  pub db: DatabaseConnection,
}

impl AppState {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }
}

pub async fn run(router: Router<AppState>) -> ApiResult<()> {
  logger::init().await?;
  id::init().await?;
  tracing::info!("Starting app server...");

  let db = database::init().await?;
  let state = AppState::new(db);
  let server = server::Server::new(config::get().server());

  server.start(state, router).await?;

  Ok(())
}
