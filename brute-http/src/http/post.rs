use actix::Addr;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use dotenvy::var;
use serde::Deserialize;
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::{model::Individual, system::BruteSystem, validator::Validate};

pub fn post_router() -> Router {
    let bearer_token = var("BEARER_TOKEN").unwrap();
    Router::new()
        .route("/attack/add", post(post_add_attack))
        .layer(ValidateRequestHeaderLayer::bearer(&bearer_token))
}

/////////////
/// POST ///
/////////////////////////
/// brute/attack/add ///
///////////////////////
#[derive(Deserialize)]
struct IndividualPayload {
    username: String,
    password: String,
    ip_address: String,
    protocol: String,
}
async fn post_add_attack(
    Extension(actor): Extension<Addr<BruteSystem>>,
    Json(payload): Json<IndividualPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let individual = Individual::new_short(
        payload.username,
        payload.password,
        payload.ip_address,
        payload.protocol,
    );
    individual.validate()?;
    match actor.send(individual).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong on our side.".to_string(),
        )),
    }
}
