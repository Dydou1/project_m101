use actix_web::{
    HttpResponse, Responder, get,
    web::{ServiceConfig, scope},
};
use sqlx::query_file_as;

use crate::models::Node;
use crate::{db::POOL, or_fail};

pub fn configure(cfg: &mut ServiceConfig) {
    let service = scope("nodes").service(get);

    cfg.service(service);
}

#[get("")]
async fn get() -> impl Responder {
    let nodes = or_fail!(
        query_file_as!(Node, "src/nodes/get.sql")
            .fetch_all(&*POOL)
            .await
    );

    HttpResponse::Ok().json(nodes)
}
