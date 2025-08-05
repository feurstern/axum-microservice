use crate::{
    db::database::DbPool,
    models::user::{NewUser, User},
};
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    if !new_user.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success" : false, "message":"Invalid email address" })),
        ));
    }

    let existing_user = sqlx::query!(r#"SELECT id from users where email=$1"#, new_user.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({"success" : false, "message"  : format!("Something went wrong : {}",e)}),
                ),
            )
        })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false,  "message" :"Email is already taken!"})),
        ));
    }

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, first_name, last_name, password, role_id, is_verified)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, first_name, last_name, password, role_id, is_verified, created_at, updated_at, deleted_at
        "#,
        new_user.email,
        new_user.first_name,
        new_user.last_name,
        new_user.password,
        new_user.role_id,
        new_user.is_verified,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
         
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    Ok(Json(user))
}

pub async fn delete_user(State(pool): State<Pool<Postgres>>, user_id: i32) {}
