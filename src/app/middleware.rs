use crate::app::auth::{JWT, get_jwt};
use crate::app::errors::ApiError;
use axum::body::Body;
use axum::http::{Request, Response, header};
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
  LazyLock::new(|| AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_jwt())));

#[derive(Debug, Clone)]
pub struct JWTAuth {
  jwt: &'static JWT,
}

impl JWTAuth {
  pub fn new(jwt: &'static JWT) -> Self {
    Self { jwt }
  }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
  type RequestBody = Body;
  type ResponseBody = Body;
  type Future = Pin<
    Box<
      dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
        + Send
        + 'static,
    >,
  >;

  fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
    let jwt = self.jwt;

    Box::pin(async move {
      let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .map(|value| -> Result<_, ApiError> {
          let token = value
            .to_str()
            .map_err(|_| {
              ApiError::Unauthenticated(String::from("Authorization header is not a valid string"))
            })?
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
              ApiError::Unauthenticated(String::from(
                "Authorization header must start with 'Bearer '",
              ))
            })?;

          Ok(token)
        })
        .transpose()?
        .ok_or_else(|| {
          ApiError::Unauthenticated(String::from("Authorization header is required"))
        })?;

      let principal = jwt.decode(token)?;
      request.extensions_mut().insert(principal);

      Ok(request)
    })
  }
}

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
  &AUTH_LAYER
}
