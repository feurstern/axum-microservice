use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        Ok(Config {
            database_url: env::var("database_url")?,
            server_port: env::var("server_port")?.parse::<u16>().unwrap(),
        })
    }
}
