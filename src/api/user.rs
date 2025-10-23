use crate::app::AppState;
use crate::app::common::{Page, PaginationParams};
use crate::app::enumeration::Gender;
use crate::app::errors::{ApiError, ApiResult};
use crate::app::path::Path;
use crate::app::response::ApiResponse;
use crate::app::utils::encode_password;
use crate::app::valid::{ValidJson, ValidQuery};
use crate::entity::prelude::*;
use crate::entity::sys_user;
use crate::entity::sys_user::ActiveModel;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
  Router::new()
    .route("/", routing::get(find_page))
    .route("/", routing::post(create))
    .route("/{id}", routing::put(update))
    .route("/{id}", routing::delete(delete))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
  keyword: Option<String>,
  #[validate(nested)]
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[debug_handler]
async fn find_page(
  State(AppState { db }): State<AppState>,
  ValidQuery(UserQueryParams {
    keyword,
    pagination,
  }): ValidQuery<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<sys_user::Model>>> {
  let paginator = SysUser::find()
    .apply_if(keyword.as_ref(), |query, keyword| {
      query.filter(
        Condition::any()
          .add(sys_user::Column::Name.contains(keyword))
          .add(sys_user::Column::Account.contains(keyword)),
      )
    })
    .order_by_desc(sys_user::Column::CreatedAt)
    .paginate(&db, pagination.size);

  let total = paginator.num_items().await?;
  let items = paginator.fetch_page(pagination.page - 1).await?;
  let page = Page::from_pagination(pagination, total, items);

  Ok(ApiResponse::ok("ok", Some(page)))
}
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
  #[validate(length(min = 1, max = 16, message = "Name length must be between 1 and 16"))]
  pub name: String,
  pub gender: Gender,
  #[validate(length(min = 1, max = 16, message = "Account length must be between 1 and 16"))]
  pub account: String,
  #[validate(length(max = 16, message = "Password length must be between 6 and 16"))]
  pub password: String,
  #[validate(custom(function = "crate::app::validation::is_mobile_phone"))]
  pub mobile_phone: String,
  pub birthday: Date,
  #[serde(default)]
  pub enabled: bool,
}

#[tracing::instrument(name = "create", skip_all)]
#[debug_handler]
async fn create(
  State(AppState { db }): State<AppState>,
  ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<sys_user::Model>> {
  if params.password.is_empty() {
    return Err(ApiError::Biz(String::from("Password cannot be empty")));
  }
  let mut active_model = params.into_active_model();

  let password = active_model
    .password
    .take()
    .ok_or_else(|| ApiError::Biz(String::from("Password is missing")))?;

  active_model.password = ActiveValue::Set(encode_password(&password)?);

  let result = active_model.insert(&db).await?;

  Ok(ApiResponse::ok("ok", Some(result)))
}

#[tracing::instrument(name = "update", skip_all)]
#[debug_handler]
async fn update(
  State(AppState { db }): State<AppState>,
  Path(id): Path<String>,
  ValidJson(params): ValidJson<UserParams>,
) -> ApiResult<ApiResponse<sys_user::Model>> {
  // 检查用户是否存在
  let existed_user = SysUser::find_by_id(&id)
    .one(&db)
    .await?
    .ok_or_else(|| ApiError::Biz(String::from("User to be updated does not exist")))?;

  let old_password = existed_user.password.clone();
  let password = params.password.clone();

  // 构造 ActiveModel
  let mut existed_active_model = existed_user.into_active_model();
  let mut active_model = params.into_active_model();

  // 只更新允许变动的字段
  existed_active_model.name = active_model.name;
  existed_active_model.gender = active_model.gender;
  existed_active_model.account = active_model.account;
  existed_active_model.mobile_phone = active_model.mobile_phone;
  existed_active_model.birthday = active_model.birthday;
  existed_active_model.enabled = active_model.enabled;

  existed_active_model.id = ActiveValue::Unchanged(id);

  // 处理密码逻辑
  if password.is_empty() {
    existed_active_model.password = ActiveValue::Unchanged(old_password);
  } else {
    let raw_password = active_model
      .password
      .take()
      .ok_or_else(|| ApiError::Biz(String::from("Password is missing")))?;
    existed_active_model.password = ActiveValue::Set(encode_password(&raw_password)?);
  }

  // 执行更新
  let result = existed_active_model.update(&db).await?;

  Ok(ApiResponse::ok("ok", Some(result)))
}

#[tracing::instrument(name = "delete", skip_all)]
#[debug_handler]
async fn delete(
  State(AppState { db }): State<AppState>,
  Path(id): Path<String>,
) -> ApiResult<ApiResponse<()>> {
  let existed_user = SysUser::find_by_id(&id)
    .one(&db)
    .await?
    .ok_or_else(|| ApiError::Biz(String::from("User to be deleted does not exist")))?;
  let result = existed_user.delete(&db).await?;
  tracing::info!(
    "Deleted user: {}, affected rows: {}",
    id,
    result.rows_affected
  );

  Ok(ApiResponse::ok("ok", None))
}

#[tracing::instrument(name = "QueryUsers", skip_all)]
#[debug_handler]
async fn query_users(
  State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
  let users = SysUser::find()
    .filter(
      Condition::all()
        .add(sys_user::Column::Gender.eq("male"))
        .add(sys_user::Column::Name.starts_with("B")),
    )
    .all(&db)
    .await?;

  Ok(ApiResponse::ok("ok", Some(users)))
}
