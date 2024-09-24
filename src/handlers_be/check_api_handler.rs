use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EchoRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    echoed_value: String,
}

pub async fn check(data: web::Json<EchoRequest>) -> impl Responder {
    let response = EchoResponse {
        echoed_value: data.value.clone(),
    };

    HttpResponse::Ok().json(response)
}
