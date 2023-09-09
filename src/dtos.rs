use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::User;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct RegisterUserDto {
  #[validate(length(min = 1, message = "Name is required"))]
  pub name: String,
  #[validate(
    length(min = 1, message = "Email is requied"),
    email(message = "Email is invalid")
  )]
  pub email: String,
  #[validate(
    length(min = 1, message = "Password is required"),
    length(min = 6, message = "Password is must be at least 6 character")
  )]
  pub password: String,
  #[validate(
    length(min = 1, message = "Please confirm your password"),
    must_match(other = "password", message = "Passwords do not match")
  )]
  #[serde(rename = "passwordConfirm")]
  pub password_confirm: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LoginUserDto {
  #[validate(
    length(min = 1, message = "Email is required"),
    email(message = "Email is invalid")
  )]
  pub email: String,
  #[validate(
    length(min = 1, message = "Password is required"),
    length(min = 6, message = "Password must be at least 6 characters")
  )]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQueryDto {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
  pub id: String,
  pub name: String,
  pub email: String,
  pub role: String,
  pub photo: String,
  pub verified: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
  pub fn filter_user(user: &User) -> Self {
    FilterUserDto {
      id: user.id.to_string(),
      name: user.name.to_string(),
      email: user.email.to_string(),
      role: user.role.to_str().to_string(),
      photo: user.photo.to_string(),
      verified: user.verified,
      created_at: user.created_at.unwrap(),
      updated_at: user.updated_at.unwrap(),
    }
  }

  pub fn filter_users(users: &[User]) -> Vec<FilterUserDto> {
    users.iter().map(FilterUserDto::filter_user).collect()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
  pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
  pub status: String,
  pub data: UserData,
}

pub struct UserListResponseDto {
  pub status: String,
  pub users: Vec<FilterUserDto>,
  pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
  pub status: String,
  pub token: String,
}
