#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod repository;
mod schema;
mod translation;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("No endpoint here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8081";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(index)
            .service(
                web::scope("/api")
                    .service(translation::get_translations)
                    .service(translation::get_translation)
                    .service(translation::add_translation)
                    .service(translation::update_translation)
                    .service(translation::delete_translation),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
