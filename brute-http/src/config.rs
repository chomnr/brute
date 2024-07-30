/// The configuration  parameters in order to run Brute successfully.
#[derive(clap::Parser)]
pub struct Config {
    /// Database connection string.
    #[clap(long, env)]
    pub database_url: String,

    /// API token for IPinfo.io service.
    #[clap(long, env)]
    pub ipinfo_token: String
}