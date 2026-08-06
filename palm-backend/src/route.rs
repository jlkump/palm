use std::sync::Arc;

/// Every API endpoint in our backend (as defined in the main function)
/// points to a function in this file.
/// 
/// NOTE:
///   For the future, it may make sense to break up these functions across different files
///   if this file gets too large.
use axum::{Json, extract::{Path, State}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, database::{self, User}};


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserCreation {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub services: Vec<String>,
}

/// Used to create a user across all 
/// the provided services, always including Palm.
/// 
/// We create the user across all services.
/// We attempt some number of times for each service. If all fail for a service,
/// we skip it and make sure to communicate which services were successful in registration.
/// 
/// The creation only fails as a whole if the Palm user can not be created.
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCreation>,
) -> String {
    // First attempt creation of the Palm user in our Postgres database
    //      Return any error codes
    match database::create_user(&state.db_pool, &payload).await {
        Ok(_) => "Success!".to_string(),
        Err(e) => format!("Failed: {}", e.to_string()),
    }

    // Then, attempt creation across all the services requested

    // Record all services that failed some number of attempts
    // Return the error codes
    // Front-end will have to handle failures
}

pub async fn update_user(Path(user_id): Path<Uuid>) {
    todo!()
}

pub async fn get_user(Path(user_id): Path<Uuid>) {
    todo!()
}

pub async fn get_users(
    State(state): State<Arc<AppState>>
) -> Json<Vec<User>> {
    Json(database::get_users(&state.db_pool).await)
}

pub async fn create_user_service(Path((user_id, service_name)): Path<(Uuid, String)>) {
    todo!()
}

pub async fn get_user_service(Path((user_id, service_name)): Path<(Uuid, String)>) {
    todo!()
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserSync {
    pub service_id: String,
    pub service_user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub archived: bool,
}

pub async fn sync_user_service(
    Path((user_id, service_name)): Path<(Uuid, String)>,
    Json(payload): Json<UserSync>
) -> Json<UserSync> {
    Json(payload)
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserDelete {
    pub service_user_id: String,
}

pub async fn delete_user_service(
    Path((user_id, service_name)): Path<(Uuid, String)>,
    Json(payload): Json<UserDelete>
) {
    todo!()
}

pub async fn delete_user(
    Path(user_id): Path<Uuid>
) {
    todo!()
}

pub async fn get_services() {

}