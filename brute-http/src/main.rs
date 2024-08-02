use actix::Actor;
use anyhow::Result;
use axum_server::tls_rustls::RustlsConfig;
use brute_http::{config::Config, http::serve_tls, system::BruteSystem};
use clap::Parser;
use dotenvy::var;
use ipinfo::{IpInfo, IpInfoConfig};
use log::info;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::path::{Path, PathBuf};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().unwrap();

    // Initialize logger
    env_logger::builder()
        .filter_module("async_io", log::LevelFilter::Off)
        .filter_module("async_std", log::LevelFilter::Off)
        .filter_module("polling", log::LevelFilter::Off)
        .filter_module("tracing", log::LevelFilter::Off)
        .filter_module("sqlx", log::LevelFilter::Off)
        .filter_module("tower_http", log::LevelFilter::Off)
        .filter_module("tower::buffer::worker", log::LevelFilter::Off)
        .filter_module("tower::buffer::service", log::LevelFilter::Off)
        .filter_module("hyper", log::LevelFilter::Off)
        .init();

    // Parse command-line arguments and environment variables to create a Config instance
    let config = Config::parse();

    // Create a new connection pool for PostgreSQL with a maximum of 500 connections
    let db = PgPoolOptions::new()
        .max_connections(500)
        .connect(&config.database_url)
        .await
        .map_err(|e| format!("Failed to connect to the database: {}", e))?;

    // Perform database migrations
    let migration_path = if var("RUNNING_IN_DOCKER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false)
    {
        Path::new("migrations")
    } else {
        Path::new("..\\migrations\\")
    };

    Migrator::new(migration_path)
        .await
        .map_err(|e| format!("Failed to create migrator: {}", e))?
        .run(&db)
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    info!("Migration process completed successfully.");

    // Create IpInfo client
    let ipinfo_config = IpInfoConfig {
        token: Some(config.ipinfo_token.to_string()),
        ..Default::default()
    };
    let ipinfo_client =
        IpInfo::new(ipinfo_config).map_err(|e| format!("Failed to create IpInfo client: {}", e))?;

    // Setup actor
    let brute_system = BruteSystem::new_brute(db, ipinfo_client).await;
    let brute_actor = brute_system.start();

    // Load TLS configuration
    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("key.pem"),
    )
    .await
    .map_err(|e| format!("Failed to load TLS configuration: {}", e))?;

    // Serve TLS
    serve_tls(brute_actor, tls_config, axum_server::Handle::new())
        .await
        .map_err(|e| format!("Failed to start TLS server: {}", e))?;
    // use serve(actor) if you don't want to use tls.
    Ok(())
}
