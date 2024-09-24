use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::sync::Arc;

mod routes;
mod handlers_be;
mod handlers_fe;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection().await;
    let pool = Arc::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}