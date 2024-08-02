use protocol::ssh::listen_on_ssh;

mod protocol;
mod endpoint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    //let d = Daemonize::new();
    //println!("Hello, world!");
    listen_on_ssh().await?;
    Ok(())
}
