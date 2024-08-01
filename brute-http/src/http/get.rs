use std::{env::var, time::Duration};
use actix::Addr;
use axum::{
    error_handling::HandleErrorLayer, extract::Query, http::StatusCode, routing::get, Extension,
    Json, Router,
};
use serde::Deserialize;
use tower::{
    buffer::BufferLayer, limit::RateLimitLayer, BoxError, ServiceBuilder
};
use tower_http::compression::CompressionLayer;

use crate::{
    model::ProcessedIndividual,
    system::{BruteSystem, RequestWithLimit},
};

pub fn get_router() -> Router {
    let rate_limit: u64 = var("RATE_LIMIT").unwrap().parse().unwrap();
    let rate_limit_duration: u64 = var("RATE_LIMIT_DURATION").unwrap().parse().unwrap();
    Router::new().route("/attack", get(get_attacker)).layer(
        ServiceBuilder::new()
            // https://github.com/tokio-rs/axum/discussions/987
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(rate_limit, Duration::from_secs(rate_limit_duration)),
    )).layer(CompressionLayer::new())
}

#[derive(Debug, Deserialize)]
struct LimitParameter {
    limit: Option<usize>,
}

////////////
/// GET ///
///////////////////////////////////////////
/// brute/stats/attacks?limit={amount} ///
/////////////////////////////////////////
async fn get_attacker(
    Extension(actor): Extension<Addr<BruteSystem>>,
    Query(params): Query<LimitParameter>,
) -> Result<Json<Vec<ProcessedIndividual>>, StatusCode> {
    let limit = params.limit.unwrap_or(50);
    let mut request = RequestWithLimit {
        table: ProcessedIndividual::default(),
        limit,
        max_limit: 50,
    };
    if limit > request.max_limit {
        request.limit = request.max_limit;
    }
    match actor.send(request).await {
        Ok(result) => Ok(axum::Json(result?)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
