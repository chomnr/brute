mod brute;
mod http;
mod model;
mod error;

use std::{env::var, sync::Arc};

use actix::{Actor, System};
use anyhow::Result;

use brute::BruteSystem;
use http::serve;
use ipinfo::{IpInfo, IpInfoConfig};
use parking_lot::Mutex;
use sqlx::postgres::PgPoolOptions;

fn main() -> Result<()>  {
    let system = System::new();
    system.block_on(async {
        // sqlx
        let pg_conn_string = var("BRUTE_DATABASE_URL").unwrap();
        let pg_pool = PgPoolOptions::new().max_connections(200)
            .connect(&pg_conn_string).await.unwrap();

        // ipinfo
        let ipinfo_access_token = var("BRUTE_IPINFO_TOKEN").unwrap();
        let ipinfo_config = IpInfoConfig {
            token: Some(ipinfo_access_token),
            ..Default::default()
        };
        let ipinfo = IpInfo::new(ipinfo_config).unwrap();

        // actix
        let actor = BruteSystem::new(pg_pool, Arc::new(Mutex::new(ipinfo))).start();
        
        // axum
        serve(actor).await.unwrap();
    });
    Ok(())
}

// notes.
// if the same ip hits the server then just reuse the ipinfo data
// if the same ip hits an hour later then regrab the data this is to
// keep the most accurate data.