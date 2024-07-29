use std::{env::var, net::SocketAddr};

use actix::Addr;
use anyhow::Context;
use axum::{Extension, Router};
use get::get_router;
use post::post_router;
use tokio::net::TcpListener;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

use crate::brute::BruteSystem;

mod get;
mod post;

pub async fn serve(brute: Addr<BruteSystem>) -> anyhow::Result<()> {
    // environment variables
    // let bearer_token = var("BRUTE_BEARER_TOKEN")?;
    let listen_on = var("BRUTE_LISTEN_ADDRESS")?;

    // router
    let app = api_router()
        // Add a layer to access the instance of Brute
        .layer(Extension(brute))
        // Add a layer to limit the size of the request body to 60 KB
        .layer(RequestBodyLimitLayer::new(60 * 1024))
        // Add a layer for tracing HTTP requests
        .layer(TraceLayer::new_for_http());

    // http server
    let tcp_listener = TcpListener::bind(listen_on).await.unwrap();
    axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("Could not run HTTP Server")
}


fn api_router() -> Router {
    let router = Router::new();
    router.merge(get_router()).merge(post_router())
}