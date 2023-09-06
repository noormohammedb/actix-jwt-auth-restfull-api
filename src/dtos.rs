use chrono::{DateTime, Utc};

use crate::models::User;

pub struct RegisterUserDto {
  pub name: String,
  pub email: String,
  pub password: String,
  pub password_confirm: String,
}

pub struct LoginUserDto {
  pub email: String,
  pub password: String,
}

pub struct RequestQueryDto {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}

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

pub struct UserData {
  pub user: FilterUserDto,
}

pub struct UserListResponseDto {
  pub status: String,
  pub users: Vec<FilterUserDto>,
  pub results: usize,
}

pub struct UserLoginResponseDto {
  pub status: String,
  pub token: String,
}
