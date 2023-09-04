use actix_web::{get, App, HttpServer, Responder};
mod config;
mod models;
mod  db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  HttpServer::new(|| App::new().service(health_check))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn health_check() -> impl Responder {
  "working".to_owned()
}
