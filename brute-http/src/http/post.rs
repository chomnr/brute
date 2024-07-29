use std::env::var;

use actix::Addr;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::{
    brute::{self, BruteSystem},
    error::ErrorMessage,
    model::Individual,
};

pub fn post_router() -> Router {
    let bearer_token = var("BRUTE_BEARER_TOKEN").unwrap();
    Router::new()
        .route("/brute/attacker/add", post(post_add_attack))
        .layer(ValidateRequestHeaderLayer::bearer(&bearer_token))
}

async fn post_add_attack(
    Extension(actor): Extension<Addr<BruteSystem>>,
    Json(payload): Json<Individual>,
) -> Result<StatusCode, (StatusCode, Json<ErrorMessage>)> {
    match brute::Request::<Individual>::new(payload) {
        Ok(request) => {
            let decluttered_request = actor.send(request).await.unwrap();
            match decluttered_request {
                Ok(status_code) => {
                    return Ok(status_code);
                },
                Err(err) => {
                    return Err(err.to_response_with_message_only())
                },
            };
        }
        Err(err) => Err(err.to_response_with_message_only()),
    }
}
