use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use serde_json::json;

use crate::{
  dtos::{FilterUserDto, UserData, UserResponseDto},
  error::HttpError,
  extractors::auth::RequireAuth,
  models::User,
};

pub fn user_scope() -> Scope {
  web::scope("/api/users").route("/me", web::get().to(get_me).wrap(RequireAuth))
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
