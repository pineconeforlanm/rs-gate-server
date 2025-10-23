use crate::app::ApiResult;
use crate::app::errors::ApiError;
use chrono::{Datelike, Utc};
use idgenerator::{IdGeneratorOptions, IdInstance};
use sea_orm::prelude::Date;

pub async fn init() -> ApiResult<()> {
  let utc_time = Utc::now();

  let base_time = Date::from_ymd_opt(utc_time.year(), utc_time.month(), utc_time.day())
    .ok_or(ApiError::Internal("Invalid base date".into()))?
    .and_hms_opt(0, 0, 0)
    .ok_or(ApiError::Internal("Invalid base datetime".into()))?
    .and_utc()
    .timestamp_millis();

  let options = IdGeneratorOptions::new()
    .base_time(base_time)
    .worker_id(1)
    .worker_id_bit_len(4);

  IdInstance::init(options).map_err(|e| ApiError::StdError(Box::new(e)))?;

  Ok(())
}

#[allow(dead_code)]
pub fn next_id() -> String {
  IdInstance::next_id().to_string()
}
