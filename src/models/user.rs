use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role_id: i32,
    pub is_verified: Option<bool>, // Nullable to match database
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role_id: i32,
    pub is_verified: bool,
}