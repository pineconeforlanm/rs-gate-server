use crate::app::auth::{Principal, get_jwt};
use crate::app::errors::ApiError;
use crate::app::middleware::get_auth_layer;
use crate::app::response::ApiResponse;
use crate::app::utils::verify_password;
use crate::app::valid::ValidJson;
use crate::app::{ApiResult, AppState};
use crate::entity::prelude::*;
use crate::entity::sys_user;
use axum::extract::{ConnectInfo, State};
use axum::{Extension, Router, debug_handler, routing};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
  Router::new()
    .route("/user-info", routing::get(get_user_info))
    .route_layer(get_auth_layer())
    .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
  #[validate(length(
    min = 3,
    max = 16,
    message = "Account length must be between 3 and 16 characters"
  ))]
  account: String,
  #[validate(length(
    min = 6,
    max = 16,
    message = "Password length must be between 6 and 16 characters"
  ))]
  password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
  access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.account, ip = %addr.ip()))]
async fn login(
  State(AppState { db }): State<AppState>,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<ApiResponse<LoginResult>> {
  tracing::info!("Processing login request...");

  let user = SysUser::find()
    .filter(sys_user::Column::Account.eq(&params.account))
    .one(&db)
    .await?
    .ok_or_else(|| ApiError::Biz(String::from("Invalid account or password")))?;

  if !verify_password(&params.password, &user.password)? {
    return Err(ApiError::Biz(String::from("Invalid account or password")));
  }

  let access_token = get_jwt().encode(Principal {
    id: user.id,
    name: user.name,
  })?;

  tracing::info!("Login successful, JWT Token: {access_token}");

  Ok(ApiResponse::ok(
    "Login successful",
    Some(LoginResult { access_token }),
  ))
}

#[debug_handler]
async fn get_user_info(
  Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>> {
  Ok(ApiResponse::ok("OK", Some(principal)))
}
