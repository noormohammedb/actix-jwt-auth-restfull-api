use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, web, App, HttpServer, Responder};
use config::Config;
use db::DBClient;
use sqlx::postgres::PgPoolOptions;

mod config;
mod db;
mod dtos;
mod error;
mod extractors;
mod models;
mod scopes;
mod utils;

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

  match sqlx::migrate!("./migrations").run(&pool).await {
    Ok(_) => println!("Migrations executed successfully."),
    Err(e) => eprintln!("Error executing migrations: {}", e),
  }

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
    let cors = Cors::default()
      .allowed_origin("http://localhost:3000")
      .allowed_origin("http://localhost:8000")
      .allowed_origin("http://localhost:8080")
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        header::ACCEPT,
      ])
      .supports_credentials();
    App::new()
      .app_data(web::Data::new(app_state.clone()))
      .wrap(cors)
      .wrap(Logger::default())
      .service(scopes::auth::auth_scope())
      .service(scopes::users::user_scope())
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
