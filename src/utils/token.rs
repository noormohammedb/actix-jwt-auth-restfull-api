use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::error::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
  pub sub: String,
  pub iat: usize,
  pub exp: usize,
}

pub fn create_token(
  user_id: &str,
  secret: &[u8],
  expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
  if user_id.is_empty() {
    return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
  }

  let now = Utc::now();
  let iat = now.timestamp() as usize;
  let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
  let claims = TokenClaims {
    sub: user_id.to_string(),
    exp,
    iat,
  };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret),
  )
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
  let decoded = decode::<TokenClaims>(
    &token.into(),
    &DecodingKey::from_secret(secret),
    &Validation::new(jsonwebtoken::Algorithm::HS256),
  );

  match decoded {
    Ok(token) => Ok(token.claims.sub),
    Err(e) => {
      dbg!(e);
      Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), 402))
    }
  }
}
