use axum::{Router, response::Html, routing::get};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tokio;

use crate::{config::config::Config, db::database::establish_connection};

mod api;
mod config;
mod db;
mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env().expect("Failed to load the configurations");

    // let pool = establish_connection(&config.database_url);

    let pool = Pool::<Postgres>::connect(&config.database_url).await?;

    let app: Router = Router::new().merge(other).with_state(pool);
    

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
