use actix_web::{HttpResponse, Responder};
use serde::Serialize;

pub async fn home() -> impl Responder {
    let response = HomeResponse {
        hello: "world".to_string(),
    };
    // 返回 JSON 响应
    HttpResponse::Ok().json(response)
}

#[derive(Serialize)]
struct HomeResponse {
    hello: String,
}