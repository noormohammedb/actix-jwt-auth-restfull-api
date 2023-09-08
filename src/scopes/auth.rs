use actix_web::{
  cookie::{time::Duration as ActixWebDuration, Cookie},
  web, HttpResponse, Responder, Scope,
};
use futures_util::TryFutureExt;
use validator::Validate;

use crate::{
  db::UserExt,
  dtos::{FilterUserDto, LoginUserDto, RegisterUserDto, UserLoginResponseDto},
  error::{ErrorMessage, HttpError},
  utils::{password, token},
  AppState,
};

pub fn auth_scope() -> Scope {
  web::scope("/api/auth")
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(signup))
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

pub async fn signup(
  state: web::Data<AppState>,
  body: web::Json<RegisterUserDto>,
) -> impl Responder {
  dbg!(&body);

  let result = state
    .db_client
    .get_user(None, None, Some(&body.email))
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

  if body.password != body.password_confirm {
    Err(HttpError::bad_request(ErrorMessage::WrongCredentials))
  } else if result.is_some() {
    Err(HttpError::bad_request(ErrorMessage::EmailExist))
  } else {
    let hashed_password = password::hash(&body.password).map_err(|e| HttpError::bad_request(e))?;

    let result = state
      .db_client
      .save_user(&body.name, &body.email, &hashed_password)
      .await
      .map_err(|e| HttpError::server_error(e.to_string()))?;

    let filterd_user = FilterUserDto::filter_user(&result);

    Ok(HttpResponse::Ok().json(filterd_user))

    // Ok("signup".to_owned())
  }
}
