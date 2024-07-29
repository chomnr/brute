mod brute;
mod http;
mod model;
mod error;

use std::{env::var, sync::Arc};

use actix::{Actor, System};
use anyhow::Result;

use brute::{BruteConfig, BruteSystem};
use http::serve;
use ipinfo::{IpInfo, IpInfoConfig};
use parking_lot::Mutex;
use sqlx::postgres::PgPoolOptions;

fn main() -> Result<()>  {
    let system = System::new();
    system.block_on(async {
        // brute p1
        let brute_config = BruteConfig::default();

        // sqlx
        let pg_pool = PgPoolOptions::new().max_connections(200)
            .connect(&brute_config.conn_string).await.unwrap();

        // ipinfo
        let ipinfo_config = IpInfoConfig {
            token: Some(brute_config.ipinfo_token.clone()),
            ..Default::default()
        };
        let ipinfo = IpInfo::new(ipinfo_config).unwrap();

        // brute p2 /w actor
        let brute_system = BruteSystem::new(pg_pool, Arc::new(Mutex::new(ipinfo)));
        let actor = brute_system.start();
        
        // axum
        serve(actor).await.unwrap();
    });
    Ok(())
}

// notes.
// if the same ip hits the server then just reuse the ipinfo data
// if the same ip hits an hour later then regrab the data this is to
// keep the most accurate data.