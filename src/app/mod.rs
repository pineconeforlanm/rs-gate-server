use crate::config;
use axum::{Router, debug_handler, routing};
pub use errors::ApiResult;
use tokio::net::TcpListener;
mod database;
pub mod errors;
mod id;
pub mod logger;
mod response;

use crate::entity::prelude::*;
use crate::entity::sys_user;
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::Condition;
use sea_orm::prelude::*;

pub async fn run() -> ApiResult<()> {
  logger::init().await?;
  id::init().await?;
  tracing::info!("Starting app server...");
  let db = database::init().await?;

  let router = Router::new()
    .route("/users", routing::get(query_users))
    .with_state(db);

  let address =
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, config::get().server().port());
  let listener = TcpListener::bind(address).await?;
  tracing::info!("Listening on {}", address);

  axum::serve(listener, router).await?;

  Ok(())
}

#[debug_handler]
async fn query_users(State(db): State<DatabaseConnection>) -> ApiResult<impl IntoResponse> {
  let users = SysUser::find()
    .filter(
      Condition::all()
        .add(sys_user::Column::Gender.eq("Male"))
        .add(sys_user::Column::Name.starts_with("B")),
    )
    .all(&db)
    .await?;

  Ok(axum::Json(users))
}
