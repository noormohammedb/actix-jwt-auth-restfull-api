use std::rc::Rc;

use actix_web::{
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
  http, web, HttpMessage,
};
use futures_util::{
  future::{ready, LocalBoxFuture, Ready},
  FutureExt,
};
use uuid::Uuid;

use crate::{
  db::UserExt,
  error::{ErrorMessage, ErrorResponse, HttpError},
  models::{User, UserRole},
  utils, AppState,
};

pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
  S: Service<
      ServiceRequest,
      Response = ServiceResponse<actix_web::body::BoxBody>,
      Error = actix_web::Error,
    > + 'static,
{
  type Response = ServiceResponse<actix_web::body::BoxBody>;
  type Error = actix_web::Error;
  type Transform = AuthMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(AuthMiddleware {
      service: Rc::new(service),
      allowed_roles: vec![UserRole::User, UserRole::Moderator, UserRole::Admin],
    }))
  }
}

pub struct RequireOnlyAdmin;

impl<S> Transform<S, ServiceRequest> for RequireOnlyAdmin
where
  S: Service<
      ServiceRequest,
      Response = ServiceResponse<actix_web::body::BoxBody>,
      Error = actix_web::Error,
    > + 'static,
{
  type Response = ServiceResponse<actix_web::body::BoxBody>;
  type Error = actix_web::Error;
  type Transform = AuthMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(AuthMiddleware {
      service: Rc::new(service),
      allowed_roles: vec![UserRole::Admin],
    }))
  }
}

pub struct AuthMiddleware<S> {
  service: Rc<S>,
  allowed_roles: Vec<UserRole>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
  S: Service<
      ServiceRequest,
      Response = ServiceResponse<actix_web::body::BoxBody>,
      Error = actix_web::Error,
    > + 'static,
{
  type Response = ServiceResponse<actix_web::body::BoxBody>;
  type Error = actix_web::Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

  fn poll_ready(
    &self,
    ctx: &mut core::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let token = req
      .cookie("token")
      .map(|c| c.value().to_string())
      .or_else(|| {
        req
          .headers()
          .get(http::header::AUTHORIZATION)
          .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
      });

    if token.is_none() {
      let json_error = ErrorResponse {
        status: "fail".to_string(),
        message: ErrorMessage::TokenNotProvided.to_string(),
      };
      return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
    }

    let app_state = req.app_data::<web::Data<AppState>>().unwrap();
    let user_id =
      match utils::token::decode_token(&token.unwrap(), app_state.env.jwt_secret.as_bytes()) {
        Ok(jwt_payload_user_id) => jwt_payload_user_id,
        Err(jwt_decode_error) => {
          return Box::pin(ready(Err(ErrorUnauthorized(ErrorResponse {
            status: "fail".to_string(),
            message: jwt_decode_error.message,
          }))));
        }
      };

    let cloned_app_state = app_state.clone();
    let allowed_roles = self.allowed_roles.clone();
    let srv = Rc::clone(&self.service);

    async move {
      let user_id = Uuid::parse_str(&user_id).unwrap();
      let result = cloned_app_state
        .db_client
        .get_user(Some(user_id.clone()), None, None)
        .await
        .map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;

      let user = result.ok_or(ErrorUnauthorized(ErrorResponse {
        status: "fail".to_string(),
        message: ErrorMessage::UserNoLongerExist.to_string(),
      }))?;

      if allowed_roles.contains(&user.role) {
        req.extensions_mut().insert::<User>(user);
        let res = srv.call(req).await?;
        Ok(res)
      } else {
        let json_error = ErrorResponse {
          status: "fail".to_string(),
          message: ErrorMessage::PermissionDenied.to_string(),
        };
        Err(ErrorForbidden(json_error))
      }
    }
    .boxed_local()
  }
}
