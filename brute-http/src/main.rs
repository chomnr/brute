use actix::{Actor, System};
use anyhow::Result;
use axum_server::tls_rustls::RustlsConfig;
use brute_http::{
    config::Config,
    http::{serve, serve_tls},
    system::BruteSystem,
};
use clap::Parser;
use dotenvy::var;
use ipinfo::{IpInfo, IpInfoConfig};
use log::info;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    // Start the logger based on environment configuration settings.
    env_logger::builder()
        .filter_module("async_io", log::LevelFilter::Off)
        .filter_module("async_std", log::LevelFilter::Off)
        .filter_module("polling", log::LevelFilter::Off)
        .filter_module("tracing", log::LevelFilter::Off)
        //.filter_module("sqlx", log::LevelFilter::Off)
        .filter_module("tower_http", log::LevelFilter::Off)
        .filter_module("tower::buffer::worker", log::LevelFilter::Off)
        .filter_module("tower::buffer::service", log::LevelFilter::Off)
        .filter_module("hyper", log::LevelFilter::Off)
        .init();

    // Parse command-line arguments and environment variables to
    // create a Config instance, loading configuration settings for the application.
    let config = Config::parse();

    // Create a new Actix system instance to manage
    // the Actix actor framework environment for the application.
    let system = System::new();

    info!("Initializing...");
    system.block_on(async {
        // Create a new connection pool for PostgreSQL with a maximum of
        // 500 connections Connect to the database using the URL provided in the configuration.
        let db: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
            .max_connections(500)
            .connect(&config.database_url)
            .await
            .unwrap();

        // Ensure the database is migrated correctly on startup.
        // Docker's doesn't support this sort of path so we need
        // to improvise.
        let is_docker: bool = var("RUNNING_IN_DOCKER")
            .unwrap_or(false.to_string())
            .parse()
            .unwrap();
        if !is_docker {
            //sqlx::migrate!("..\\migrations\\").run(&db).await.unwrap();
            Migrator::new(Path::new("..\\migrations\\"))
                .await
                .unwrap()
                .run(&db)
                .await
                .unwrap();
            info!("Migration process completed successfully.");
        } else {
            Migrator::new(Path::new("migrations"))
                .await
                .unwrap()
                .run(&db)
                .await
                .unwrap();
            info!("(Docker) Migration process completed successfully.");
        }

        // Create an instance of `IpInfoConfig` with the
        // provided token and default settings for other fields.
        let ipinfo_config = IpInfoConfig {
            token: Some(config.ipinfo_token.to_string()),
            ..Default::default()
        };
        let ipinfo_client = IpInfo::new(ipinfo_config).unwrap();

        // setup actor
        let brute_system = BruteSystem::new_brute(db, ipinfo_client).await;
        let brute_actor = brute_system.start(); // call .start() on brute_system

        // tls support
        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("cert.pem"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("key.pem"),
        )
        .await
        .unwrap();

        // Start listening.
        //serve(brute_actor).await.unwrap(); // non tls
        serve_tls(brute_actor, config).await.unwrap(); // tls
    });
    Ok(())
}
