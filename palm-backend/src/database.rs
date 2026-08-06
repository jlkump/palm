use std::{ops::Deref, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row, postgres::PgRow};

use crate::route::{UserCreation, UserDelete, UserSync};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash_salt: String,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(User { email: row.get(0), first_name: row.get(1), last_name: row.get(2), password_hash_salt: row.get(3) })
    }
}

// TODO: Properly error handle rather than use unwraps
pub async fn create_user(client: &PgPool, info: &UserCreation) -> Result<(), sqlx::Error>{
    // TODO: Password hash and salt
    sqlx::query("INSERT INTO Users (email, first_name, last_name, password_hash, date_pass_modified) VALUES ($1, $2, $3, $4, NOW());")
    .bind(&info.email).bind(&info.first_name).bind(&info.last_name).bind(&info.password)
    .execute(client)
    .await.map(|_| ())
}

pub async fn get_users(client: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT email, first_name, last_name, password_hash FROM Users;").fetch_all(client).await.unwrap()
}

pub async fn sync_user(client: &PgPool, info: &UserSync) {
    sqlx::query("
        UPDATE Users
        SET  email = $1, first_name = $2, last_name = $3, password_hash = $4, archived = $5 
        WHERE user_id = $6
    ")
    .bind(&info.email).bind(&info.first_name).bind(&info.last_name).bind(&info.password)
    .bind(&info.archived).bind(&info.service_user_id)
    .execute(client)
    .await.unwrap();
}

pub async fn remove_user(client: &PgPool, info: &UserDelete) {
    sqlx::query("
        DELETE FROM Users
        WHERE user_id = $1
    ").bind(&info.service_user_id).execute(client).await.unwrap();
}