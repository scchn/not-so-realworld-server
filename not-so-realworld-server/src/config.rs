#[derive(Debug, Clone, clap::Parser)]
pub struct Config {
    #[clap(env)]
    pub database_url: String,

    #[clap(env)]
    pub hmac_key: String,
}
