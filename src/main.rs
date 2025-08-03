use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
use tokio;

use crate::{config::config::Config, db::database::establish_connection};

mod api;
mod config;
mod db;
mod models;
mod services;
mod schema;

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("Failed to load the configurations");

    let pool = establish_connection(&config.database_url);

    let app: Router = Router::new().route(
        "/",
        get(|| async { Html("connected to the server!") }).with_state(pool),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
