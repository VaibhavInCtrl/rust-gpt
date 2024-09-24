use actix_web::{App, HttpServer};
mod routes;
mod handlers_be;
mod handlers_fe;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}