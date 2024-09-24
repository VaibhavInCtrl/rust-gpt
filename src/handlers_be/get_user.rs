use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::db::queries;

pub async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    match queries::get_items(pool.get_ref()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
