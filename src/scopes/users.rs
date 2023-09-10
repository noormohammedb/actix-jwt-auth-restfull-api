use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use serde_json::json;

use crate::{
  dtos::{FilterUserDto, UserData, UserResponseDto},
  extractors::auth::RequireAuth,
  models::User,
};

pub fn user_scope() -> Scope {
  web::scope("/api/users").route("/me", web::get().to(get_me).wrap(RequireAuth))
}

pub async fn get_me(req: HttpRequest) -> impl Responder {
  match req.extensions().get::<User>() {
    Some(user) => {
      dbg!(&user);
      let res_json = UserResponseDto {
        status: "success".to_owned(),
        data: UserData {
          user: FilterUserDto::filter_user(user),
        },
      };
      HttpResponse::Ok().json(res_json)
    }
    None => HttpResponse::BadRequest().json(json!({"status": "fail"})),
  }
}
