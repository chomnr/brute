mod http;
mod flags;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    //println!("Hello, world!");
    Ok(())
}

// Metrics::add("derive::33")

// notes.
// if the same ip hits the server then just reuse the ipinfo data
// if the same ip hits an hour later then regrab the data this is to
// keep the most accurate data.