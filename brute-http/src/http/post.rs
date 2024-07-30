use std::time::{SystemTime, UNIX_EPOCH};

use actix::Addr;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use dotenvy::var;
use serde::Deserialize;
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::{
    model::{Individual, IndividualBuilder},
    system::BruteSystem,
};

pub fn post_router() -> Router {
    let bearer_token = var("BEARER_TOKEN").unwrap();
    Router::new()
        .route("/brute/attacker/add", post(post_add_attack))
        .layer(ValidateRequestHeaderLayer::bearer(&bearer_token))
}

#[derive(Deserialize)]
struct IndividualPayload {
    username: String,
    password: String,
    ip: String,
    protocol: String,
}
async fn post_add_attack(
    Extension(actor): Extension<Addr<BruteSystem>>,
    Json(payload): Json<IndividualPayload>,
) -> Result<StatusCode, StatusCode> {
    let individual = IndividualBuilder::default()
        .id(uuid::Uuid::new_v4().as_simple().to_string())
        .username(payload.username)
        .password(payload.password)
        .ip(payload.ip)
        .protocol(payload.protocol)
        .timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        )
        .build()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    match actor.send(individual).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
