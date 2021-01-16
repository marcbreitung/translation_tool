use crate::models;
use crate::repository;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/translations")]
pub async fn get_translations(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
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
pub async fn get_translation(
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
pub async fn add_translation(
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
pub async fn update_translation(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewTranslation>,
    translation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let translation_id = translation_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");
    let translation = web::block(move || {
        repository::update_translation(
            translation_id,
            &form.key,
            &form.target,
            &form.language,
            &conn,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(translation))
}

#[delete("/translations/{translation_id}")]
pub async fn delete_translation(
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

    let delete_translation = models::DeleteTranslation::new(translation_id, translation); 

    Ok(HttpResponse::Ok().json(delete_translation))
}
