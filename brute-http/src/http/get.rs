use actix::Addr;
use axum::{extract::Query, http::StatusCode, routing::get, Extension, Json, Router};
use dotenvy::var;
use serde::Deserialize;

use crate::{model::ProcessedIndividual, system::{BruteSystem, RequestWithLimit}};

pub fn get_router() -> Router {
    Router::new().route("/attacks", get(get_attacker))
}

#[derive(Debug, Deserialize)]
struct LimitParameter {
    limit: Option<usize>,
}

// /brute/stats/attackers/ grab the first 50 'attackers' 


// /brute/stats/top_weekly get attacks done in the last week
// /brute/stats/top_hourly get attacks done in the last hour.
// /brute/stats/top_yearly get attacks done in the last yearly.
// /brute/stats/top_protocol?limit=50 get top protocols get last 50 protocols max should be 50 no limit just retrieve top
// // convert value to json....

// todo add websockets... maybe...

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
        Ok(result) => Ok(axum::Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}