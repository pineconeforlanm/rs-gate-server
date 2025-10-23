use crate::app::ApiResult;

mod api;
mod app;
mod config;
mod entity;
mod web;

#[tokio::main]
async fn main() -> ApiResult<()> {
  app::run(api::create_router()).await?;
  Ok(())
}
