use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_port: env::var("SERVER_PORT")?.parse::<u16>().unwrap(),
        })
    }
}
