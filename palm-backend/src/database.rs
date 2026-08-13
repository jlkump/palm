use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row, postgres::PgRow};

use crate::route::{APIError, UserCreation, UserDelete, UserUpdate};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(User { email: row.get(0), first_name: row.get(1), last_name: row.get(2) })
    }
}

impl From<sqlx::Error> for APIError {
    fn from(value: sqlx::Error) -> Self {
        APIError { error_string: value.to_string() }
    }
}

/// Returns the id of the user just created
pub async fn create_user(client: &PgPool, info: &UserCreation) -> Result<i32, sqlx::Error> {
    // TODO: Password hash & salt
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO Users (email, first_name, last_name, password_hash, date_pass_modified)
         VALUES ($1, $2, $3, $4, NOW())
         RETURNING user_id;"
    )
        .bind(&info.email)
        .bind(&info.first_name)
        .bind(&info.last_name)
        .bind(&info.password)
        .fetch_one(client)
        .await?;

    Ok(row.0)
}

pub async fn get_user(client: &PgPool, user_id: &i32) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query("SELECT email, first_name, last_name FROM Users WHERE user_id = $1;")
        .bind(user_id)
        .fetch_one(client)
        .await?;
    Ok(if row.is_empty() {
        None
    } else {
        Some(User::from_row(&row)?)
    })
}

pub async fn get_users(client: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as("SELECT email, first_name, last_name FROM Users;")
        .fetch_all(client)
        .await
}

pub async fn update_user(client: &PgPool, user_id: &i32, info: &UserUpdate) -> Result<(), sqlx::Error> {
    // TODO: Password hash & salt
    sqlx::query("
        UPDATE Users
        SET  email = $1, first_name = $2, last_name = $3, password_hash = $4, archived = $5 
        WHERE user_id = $6
    ")
        .bind(&info.email)
        .bind(&info.first_name)
        .bind(&info.last_name)
        .bind(&info.password)
        .bind(&info.archived)
        .bind(&user_id)
        .execute(client)
        .await?;
    Ok(())
}

pub async fn remove_user(client: &PgPool, info: &UserDelete) {
    sqlx::query("
        DELETE FROM Users WHERE user_id = $1
    ")
        .bind(&info.service_user_id)
        .execute(client)
        .await.unwrap();
}