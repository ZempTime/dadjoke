

// actix
use actix_web::{get, post, web, App, Error, HttpServer, HttpResponse, middleware};
use serde::{Serialize, Deserialize};
extern crate env_logger;

// diesel
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod db;

// Actix

#[derive(Debug, Serialize, Deserialize)]
struct DadJokeInput {
    text: String,
}

#[post("/dadjokes")]
async fn add_dadjoke(input: web::Json<models::NewDadjoke>) -> Result<HttpResponse, Error> {
    let conn = establish_connection();
    let dadjoke = web::block(move || db::create_dadjoke(&input.text))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(dadjoke))
}

#[get("/dadjokes")]
async fn index() -> Result<HttpResponse, Error> {

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(add_dadjoke)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
