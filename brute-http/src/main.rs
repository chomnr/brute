mod brute;
mod http;
mod model;
mod error;

use actix::{Actor, System};
use anyhow::Result;

use brute::{BruteConfig, BruteSystem};
use http::serve;

fn main() -> Result<()>  {
    let system = System::new();
    system.block_on(async {
        // brute
        let brute_config = BruteConfig::default();
        let brute_system = BruteSystem::new_brute(brute_config).await.start();

        // axum
        serve(brute_system).await.unwrap();
    });
    Ok(())
}

// notes.
// if the same ip hits the server then just reuse the ipinfo data
// if the same ip hits an hour later then regrab the data this is to
// keep the most accurate data.