use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use serde_json::json;
use validator::Validate;

use crate::{
  db::UserExt,
  dtos::{FilterUserDto, RequestQueryDto, UserData, UserListResponseDto, UserResponseDto},
  error::HttpError,
  extractors::auth::{RequireAuth, RequireOnlyAdmin},
  models::User,
  AppState,
};

pub fn user_scope() -> Scope {
  web::scope("/api/users")
    .route("", web::get().to(get_users).wrap(RequireOnlyAdmin))
    .route("/me", web::get().to(get_me).wrap(RequireAuth))
}

pub async fn get_me(req: HttpRequest) -> impl Responder {
  match req.extensions().get::<User>() {
    Some(user) => {
      let filtered_user = FilterUserDto::filter_user(user);

      let response_data = UserResponseDto {
        status: "success".to_owned(),
        data: UserData {
          user: filtered_user,
        },
      };
      Ok(HttpResponse::Ok().json(response_data))
    }
    None => Err(HttpError::server_error("User not found")),
  }
}

pub async fn get_users(
  state: web::Data<AppState>,
  query: web::Query<RequestQueryDto>,
) -> Result<HttpResponse, HttpError> {
  let query_params = query.into_inner();
  query_params
    .validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

  let offset = query_params.page.unwrap_or(1);
  let limit = query_params.limit.unwrap_or(10);

  let users = state
    .db_client
    .get_users(offset as u32, limit)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

  let response_data = UserListResponseDto {
    status: "success".to_owned(),
    results: users.len(),
    users: FilterUserDto::filter_users(&users),
  };

  Ok(HttpResponse::Ok().json(response_data))
}
