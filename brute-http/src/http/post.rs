use std::{env::var, sync::Arc};

use actix::{Actor, Addr};
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::{attacker::{Attacker, AttackerRequest}, brute::Brute, flags::Flags};

pub fn post_router() -> Router {
    let bearer_token = var("BRUTE_BEARER_TOKEN").unwrap();
    Router::new()
        .route("/brute/attacker/add", post(post_add_attack))
        .layer(ValidateRequestHeaderLayer::bearer(&bearer_token))
}

async fn post_add_attack(
    Extension(actor): Extension<Addr<Brute>>,
    Json(payload): Json<Attacker>,
) -> Result<StatusCode, (StatusCode, String)> {
    let flags = Flags::INSERT | Flags::UPDATE;
    actor.send(AttackerRequest::new(payload, flags)).await.unwrap();
    Ok(StatusCode::OK)
}
