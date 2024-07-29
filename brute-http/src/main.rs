mod brute;
mod attacker;
mod http;
mod flags;

use std::env::var;

use actix::{Actor, System};
use anyhow::Result;
use brute::Brute;

use http::serve;
use ipinfo::{IpInfo, IpInfoConfig};
use sqlx::postgres::PgPoolOptions;


fn main() -> Result<()>  {
    let system = System::new();
    system.block_on(async {
        // Create a connection pool
        let pg_conn_string = var("BRUTE_CONNECTION_STRING").unwrap();
        let pg_pool = PgPoolOptions::new().max_connections(200)
            .connect(&pg_conn_string).await.unwrap();

        // ipinfo
        let ipinfo_access_token = var("BRUTE_IPINFO_ACCESS_TOKEN").unwrap();
        let ipinfo_config = IpInfoConfig {
            token: Some(ipinfo_access_token),
            ..Default::default()
        };
        let mut ipinfo = IpInfo::new(ipinfo_config).unwrap();

        // Create and start the actor
        let actor = Brute::new(pg_pool, ipinfo).start();

        // Start the Axum server
        serve(actor).await.unwrap();
    });
    Ok(())
}

// notes.
// if the same ip hits the server then just reuse the ipinfo data
// if the same ip hits an hour later then regrab the data this is to
// keep the most accurate data.