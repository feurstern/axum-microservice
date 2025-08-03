use crate::db::database::DbPool;
use crate::models::user::{self, NewUser, UpdateUser, User};
use crate::schema::users::dsl::*;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diesel::RunQueryDsl;
use serde_json::json;

pub async fn create_user(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    let conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(user))
}
