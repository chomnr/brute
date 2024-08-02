use actix::Addr;
use axum::{Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::var;
use get::get_router;
use log::info;
use post::post_router;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

use crate::system::BruteSystem;

mod get;
mod post;

//////////////////
/// NO SSL/TLS //
////////////////
pub async fn serve(brute_actor: Addr<BruteSystem>, handle: axum_server::Handle) -> anyhow::Result<()> {
    // environment variables
    // let bearer_token = var("BRUTE_BEARER_TOKEN")?;
    let listen_on = var("LISTEN_ADDRESS")?;

    // router
    let app = api_router()
        // Add a layer to access the instance of Brute
        .layer(Extension(brute_actor))
        // Add a layer to limit the size of the request body to 3 KB
        .layer(RequestBodyLimitLayer::new(3 * 1024))
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

////////////////
/// SSL/TLS ///
//////////////
pub async fn serve_tls(brute_actor: Addr<BruteSystem>, config: RustlsConfig, handle: axum_server::Handle) -> anyhow::Result<()> {
    // environment variables
    let listen_on = var("LISTEN_ADDRESS_TLS")?;

    // router
    let app = api_router()
        // Add a layer to access the instance of Brute
        .layer(Extension(brute_actor))
        // Add a layer to limit the size of the request body to 3 KB
        .layer(RequestBodyLimitLayer::new(3 * 1024))
        // Add a layer for tracing HTTP requests
        .layer(TraceLayer::new_for_http());

    // http server

    let mut parts = listen_on.split(':');
    let ip_part = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing IP address"))?;
    let port_part = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing port"))?;

    let ip: IpAddr = ip_part.parse()?;
    let port: u16 = port_part.parse()?;


    let socket_addr = SocketAddr::new(ip, port);
    info!("(TLS) Server is now listening on {}.", listen_on);
    axum_server::bind_rustls(socket_addr, config)
        .handle(handle)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    Ok(())
}

fn api_router() -> Router {
    let router = Router::new()
        .nest("/brute", post_router())
        .nest("/brute/stats", get_router());
    router
}

mod websocket {
   
}
