use actix::System;
use anyhow::Result;
use brute_http::{config::Config, http::serve};
use clap::Parser;
use ipinfo::IpInfoConfig;
use log::info;
use sqlx::
    postgres::PgPoolOptions
;

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
        .filter_module("sqlx", log::LevelFilter::Off)
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

        // Create an instance of `IpInfoConfig` with the
        // provided token and default settings for other fields.
        let ipinfo_config = IpInfoConfig {
            token: Some(config.ipinfo_token.to_string()),
            ..Default::default()
        };
        
        // Ensure the database is migrated correctly on startup.
        sqlx::migrate!("..\\migrations\\").run(&db).await.unwrap();

        info!("Migration process completed successfully.");

        // Start listening.
        serve().await.unwrap();
    });
    Ok(())
}
