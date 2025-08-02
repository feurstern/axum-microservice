mod db;
use crate::db::establish_connection;
use axum::{Router, handler::Handler, response::Html, routing::get};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let pool = establish_connection();
    let app: Router = Router::new().route(
        "/",
        get(|| async { Html("connected to the server!") }).with_state(pool),
    );
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2121").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
