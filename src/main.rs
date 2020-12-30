#[macro_use]
extern crate diesel;

use actix_web::{
    delete, get, middleware, post, put, web, App, Error, HttpResponse, HttpServer, Responder,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod models;
mod repository;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("No endpoint here!")
}

#[get("/translations")]
async fn get_translations(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let users = web::block(move || repository::find_translations(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/translations/{translation_id}")]
async fn get_translation(
    pool: web::Data<DbPool>,
    translation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let translation_id = translation_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let translation =
        web::block(move || repository::find_translation_by_uid(translation_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    if let Some(translation) = translation {
        Ok(HttpResponse::Ok().json(translation))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No translation found with uid {}", translation_id));
        Ok(res)
    }
}

#[post("/translations")]
async fn add_translation(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewTranslation>,
) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let translation = web::block(move || {
        repository::add_translation(&form.key, &form.target, &form.language, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(translation))
}

#[put("/translations/{translation_id}")]
async fn update_translation(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewTranslation>,
    translation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let translation_id = translation_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let translation = web::block(move || {
        repository::update_translation(translation_id, &form.key, &form.target, &form.language, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(translation))
}

#[delete("/translations/{translation_id}")]
async fn delete_translation(
    pool: web::Data<DbPool>,
    translation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let translation_id = translation_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let translation = web::block(move || repository::delete_translation(translation_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let res = HttpResponse::Ok().body(format!("Deleted translations: {}", translation));

    Ok(res)
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

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_translations)
            .service(get_translation)
            .service(add_translation)
            .service(update_translation)
            .service(delete_translation)
    })
    .bind(&bind)?
    .run()
    .await
}
