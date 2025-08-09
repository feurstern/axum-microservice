use crate::services::user_service::create_user;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/user", post(create_user))
        .with_state(pool)
}
