use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use config::Config;
use db::DBClient;
use sqlx::postgres::PgPoolOptions;

mod config;
mod db;
mod error;
mod models;
mod dtos;

#[derive(Debug, Clone)]
pub struct AppState {
  pub env: Config,
  pub db_client: DBClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "actix_web=info");
  }

  dotenv::dotenv().ok();
  env_logger::init();

  let config = Config::init();

  let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&config.database_url)
    .await?;

  let db_client = DBClient::new(pool);
  let app_state: AppState = AppState {
    env: config.clone(),
    db_client,
  };

  println!(
    "{}",
    format!("Server is running on http://127.0.0.1:{}", config.port)
  );

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(app_state.clone()))
      .wrap(Logger::default())
      .service(health_check)
  })
  .bind(format!("0.0.0.0:{}", config.port))?
  .run()
  .await?;

  Ok(())
}

#[get("/")]
async fn health_check() -> impl Responder {
  "working".to_owned()
}
