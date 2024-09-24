use actix_web::web;
use crate::handlers_be::check_api_handler;
use crate::handlers_be::get_user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/check", web::post().to(check_api_handler::check))
            .route("/items", web::get().to(get_user::get_items))
    );
}
