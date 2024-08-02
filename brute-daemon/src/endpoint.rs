use std::env::var;

use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    username: String,
    password: String,
    ip_address: String,
    protocol: String,
}

pub async fn post_add_attacker(
    username: &str,
    password: &str,
    ip_address: &str,
    protocol: &str
) -> anyhow::Result<()> {
    let client = Client::new();
    let url = var("ADD_ATTACK_ENDPOINT")?;
    let bearer_token = var("BEARER_TOKEN")?;

    let payload = Payload {
        username: String::from(username),
        password: String::from(password),
        ip_address: String::from(ip_address),
        protocol: String::from(protocol),
    };
    client.post(url).bearer_auth(bearer_token).json(&payload).send().await?;
    Ok(())
}
