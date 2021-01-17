use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

use crate::models::languages::{NewLanguage, DeleteLanguage};
use crate::repositories::languages;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/languages")]
pub async fn get_languages(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");

    let result = web::block(move || languages::find_languages(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/languages/{language_id}")]
pub async fn get_language(
    pool: web::Data<DbPool>,
    language_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let language_id = language_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");

    let result =
        web::block(move || languages::find_language_by_uid(language_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No translation found with uid {}", language_id));
        Ok(res)
    }
}

#[post("/languages")]
pub async fn add_language(
    pool: web::Data<DbPool>,
    form: web::Json<NewLanguage>,
) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");

    let result = web::block(move || {
        languages::add_language(&form.name, &form.lang, &form.territory, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(result))
}

#[put("/languages/{language_id}")]
pub async fn update_language(
    pool: web::Data<DbPool>,
    form: web::Json<NewLanguage>,
    language_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let language_id = language_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");

    let result = web::block(move || {
        languages::update_language(
            language_id,
            &form.name,
            &form.lang,
            &form.territory,
            &conn,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/languages/{language_id}")]
pub async fn delete_language(
    pool: web::Data<DbPool>,
    language_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let language_id = language_id.into_inner();
    let conn = pool
        .get()
        .expect("Couldn't get database connection from pool");

    let result = web::block(move || languages::delete_language(language_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let delete_language = DeleteLanguage::new(language_id, result);

    Ok(HttpResponse::Ok().json(delete_language))
}
