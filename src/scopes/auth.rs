use actix_web::{
  cookie::{time::Duration as ActixWebDuration, Cookie},
  web, HttpResponse, Responder, Scope,
};
use futures_util::TryFutureExt;
use validator::Validate;

use crate::{
  db::UserExt,
  dtos::{LoginUserDto, UserLoginResponseDto},
  error::{ErrorMessage, HttpError},
  utils::{password, token},
  AppState,
};

pub fn auth_scope() -> Scope {
  web::scope("/api/auth").route("/login", web::post().to(login))
}

pub async fn login(state: web::Data<AppState>, body: web::Json<LoginUserDto>) -> impl Responder {
  dbg!(&body);
  body
    .validate()
    .map_err(|e| HttpError::bad_request(e.to_string()))?;

  let result = state
    .db_client
    .get_user(None, None, Some(&body.email))
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

  let user = result.ok_or(HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

  // dbg!(&user);

  let password_matches = password::compare(&body.password, &user.password)
    .map_err(|_| HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

  if password_matches {
    let token = token::create_token(
      &user.id.to_string(),
      state.env.jwt_secret.as_bytes(),
      state.env.jwt_maxage,
    )
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let cookie_with_token = Cookie::build("token", token.to_owned())
      .path("/")
      .max_age(ActixWebDuration::new(60 * &state.env.jwt_maxage, 0))
      .http_only(true)
      .finish();

    Ok(
      HttpResponse::Ok()
        .cookie(cookie_with_token)
        .json(UserLoginResponseDto {
          status: "success".to_string(),
          token,
        }),
    )
  } else {
    Err(HttpError::unauthorized(ErrorMessage::WrongCredentials))
  }

  // Ok::<std::string::String, Box<dyn std::error::Error>>( serde_json::to_string(&body.into_inner()).unwrap_or("login".to_string()))
}
