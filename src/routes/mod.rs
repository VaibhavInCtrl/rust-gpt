mod page;
mod api;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    page::config(cfg);
    api::config(cfg);
}
