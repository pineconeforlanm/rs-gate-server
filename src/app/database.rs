use crate::app::ApiResult;
use crate::app::errors::ApiError;
use crate::config;
use sea_orm::{
  ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> ApiResult<DatabaseConnection> {
  let database_config = config::get().database();
  let mut options = ConnectOptions::new(format!(
    "postgres://{}:{}@{}:{}/{}",
    database_config.user(),
    database_config.password(),
    database_config.host(),
    database_config.port(),
    database_config.database()
  ));

  let cpus = num_cpus::get() as u32;
  options
    .min_connections(max(cpus * 4, 10))
    .max_connections(max(cpus * 8, 20))
    .connect_timeout(Duration::from_secs(10))
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(300))
    .max_lifetime(Duration::from_secs(3600 * 24))
    .sqlx_logging(false)
    .set_schema_search_path(database_config.schema());

  // 将 sea_orm::DbErr 自动映射到 ApiError::Database
  let db = Database::connect(options).await?;
  db.ping().await?;
  tracing::info!("Database connected successfully");

  log_database_version(&db).await?;

  Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> ApiResult<()> {
  let version_result = db
    .query_one(Statement::from_string(
      DbBackend::Postgres,
      String::from("select version()"),
    ))
    .await?
    .ok_or_else(|| ApiError::Internal("Failed to get database version".into()))?;

  let version = version_result.try_get_by_index::<String>(0)?;

  tracing::info!("Database version: {}", version);

  Ok(())
}
