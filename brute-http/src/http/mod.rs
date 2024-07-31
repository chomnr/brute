use std::net::SocketAddr;
use actix::Addr;
use axum::{Extension, Router};
use dotenvy::var;
use get::get_router;
use log::info;
use post::post_router;
use tokio::net::TcpListener;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

use crate::system::BruteSystem;

mod get;
mod post;

pub async fn serve(brute_actor: Addr<BruteSystem>) -> anyhow::Result<()> {
    // environment variables
    // let bearer_token = var("BRUTE_BEARER_TOKEN")?;
    let listen_on = var("LISTEN_ADDRESS")?;

    // router
    let app = api_router()
        // Add a layer to access the instance of Brute
        .layer(Extension(brute_actor))
        // Add a layer to limit the size of the request body to 60 KB
        .layer(RequestBodyLimitLayer::new(60 * 1024))
        // Add a layer for tracing HTTP requests
        .layer(TraceLayer::new_for_http());

    // http server
    info!("Server is now listening on {}.", listen_on);
    let tcp_listener = TcpListener::bind(listen_on).await.unwrap();
    axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
    Ok(())
}

fn api_router() -> Router {
    let router = Router::new()
        .nest("/brute/attack", post_router())
        .nest("/brute/stats", get_router());
    router
}
