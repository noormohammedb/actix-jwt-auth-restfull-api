use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq, Default)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
  Admin,
  Moderator,
  #[default]
  User,
}

impl UserRole {
  pub fn to_str(&self) -> &str {
    match self {
      UserRole::Admin => "admin",
      UserRole::User => "user",
      UserRole::Moderator => "moderator",
    }
  }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone, Default)]
pub struct User {
  pub id: uuid::Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  pub role: UserRole,
  pub photo: String,
  pub verified: bool,
  #[serde(rename = "createdAt")]
  pub created_at: Option<DateTime<Utc>>,
  #[serde(rename = "updatedAt")]
  pub updated_at: Option<DateTime<Utc>>,
}
