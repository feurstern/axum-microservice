use crate::db::database::DbPool;
use crate::models::user::{NewUser, UpdateUser};
use crate::schema::users::dsl::*;
use axum::extract::{Path, State};

pub async fn create_user(State(pool): State<DbPool>) {}
