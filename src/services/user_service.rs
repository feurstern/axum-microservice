use crate::{
    models::user::{UpsertUser, User},
};
use axum::{Json, extract::{State, Path}, http::StatusCode};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<UpsertUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !new_user.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success" : false, "message":"Invalid email address" })),
        ));
    }


    let user_existing = sqlx::query_as::<_, User>(
        "SELECT 
            id 
           FROM 
                users 
            WHERE 
                email= $1 
            AND
                deleted_at is null
          "
    ).bind(new_user.email.clone())
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success" : false, "message" : format!("Error user existing : {}", e)})),
        )
    })?;

    if user_existing.is_some() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"success" : false, "message" : "The email is already taken!"}))));
    };

    let hashed_password = hash(&new_user.password, DEFAULT_COST).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success" : false, "message" : format!("Error hashed password: {}",e)}))
        )
    })?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, first_name, last_name, password, role_id, is_verified)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, first_name, last_name, password, role_id, is_verified, created_at, updated_at, deleted_at
        "#
    )
    .bind(new_user.email)
    .bind(new_user.first_name)
    .bind(new_user.last_name)
    .bind(hashed_password)
    .bind(new_user.role_id)
    .bind(new_user.is_verified)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
         
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
    })?;

    Ok(Json(json!({"success" : true,  "data" : user})))
}

pub async fn delete_user(State(pool): State<Pool<Postgres>>, Path(user_id): Path<i32>) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {


    let now = Utc::now().naive_utc();

    let result = sqlx::query(
        r#"
        UPDATE users 
        SET deleted_at = $1 
        WHERE id = $2
        "#
    )
    .bind(now)
    .bind(user_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "message": format!("Error when trying to delete the data: {}", e)})),
        )
    })?;

    if result.rows_affected() == 0{
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"success": false, "message": "User not found"})),
        ));
    }

    Ok(Json(json!({"success" : true, "message" : format!("user with id {}: has been deleted", user_id)})))

}



pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
    Json(update_user_payload): Json<UpsertUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {


    let hashed_password = hash(&update_user_payload.password, DEFAULT_COST).map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success" :false}))
        )
    })?;

    let result = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET email = $1,
            first_name = $2,
            last_name = $3,
            role_id = $4,
            password = $5,
            updated_at = NOW()
        WHERE id = $6
        RETURNING *
        "#
    )
    .bind(&update_user_payload.email)
    .bind(&update_user_payload.first_name)
    .bind(&update_user_payload.last_name)
    .bind(update_user_payload.role_id)
    .bind(hashed_password)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "message": format!("Error updating user: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": result
    })))
}