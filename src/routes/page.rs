use actix_web::web;
use crate::handlers_fe::hello_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fe")
            .route("/hello", web::get().to(hello_handler::hello))
    );
}
