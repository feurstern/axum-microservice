use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role_id: i32,
    pub is_verified: bool,
    pub created_at: Option<NaiveDateTime>,
}

// if there is complaint from the insertable that is not statisfied, you haven't defined the new user yet
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users )]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role_id: i32,
}
