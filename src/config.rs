#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
  pub jwt_secret: String,
  pub jwt_maxage: i64,
  pub port: u16,
}

impl Config {
  pub fn init() -> Config {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL mut be set");
    let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY mut be set");
    let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE mut be set");
    let port = std::env::var("PORT")
      .unwrap_or("8000".to_owned())
      .parse::<u16>()
      .unwrap();

    Config {
      database_url,
      jwt_secret,
      jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
      port,
    }
  }
}
