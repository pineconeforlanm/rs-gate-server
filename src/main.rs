use crate::app::ApiResult;

mod app;
mod config;
mod entity;
#[tokio::main]
async fn main() -> ApiResult<()> {
  app::run().await?;
  Ok(())
}
