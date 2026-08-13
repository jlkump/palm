use std::sync::Arc;

/// Every API endpoint in our backend (as defined in the main function)
/// points to a function in this file.
/// 
/// NOTE:
///   For the future, it may make sense to break up these functions across different files
///   if this file gets too large.
use axum::{Json, extract::{Path, State}, response::{IntoResponse, Response}};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, database::{self, User}, services::{self, Service}};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIError {
    pub error_string: String,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.error_string).into_response()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserCreation {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub services: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserCreationResult {
    pub user_id: i32,
    pub successful_services: Vec<services::UserCreationResult>,
    pub failed_services: Vec<ServiceUserCreationFailure>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ServiceUserCreationFailure {
    pub service_name: String,
    pub error_messages: Vec<String>,
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
) -> Result<Json<UserCreationResult>, APIError> {
    // First attempt creation of the Palm user in our Postgres database
    //      Return any error codes
    let user_id = match database::create_user(&state.db_pool, &payload).await {
        Ok(id) => id,
        Err(e) => return Err(APIError { error_string: format!("Database failure: {}", e.to_string()) }),
    };

    // Then, attempt creation across all the services requested
    let mut successful_services = vec![];
    let mut failed_services = vec![];
    for s in Service::get_services_from_names(&payload.services) {
        let mut attempts = 0;
    
        let mut successful_creation = None;
        let mut service_errors = vec![];
        while successful_creation.is_none() && attempts < state.config.creation_attempts {
            match s.create_user(&payload).await {
                Ok(creation_result) => {
                    successful_creation = Some(creation_result);
                },
                Err(e) => {
                    service_errors.push(e.to_string());
                },
            }
            attempts += 1;
        }

        if let Some(result) = successful_creation {
            successful_services.push(result);
        } else {
            failed_services.push(ServiceUserCreationFailure {
                service_name: s.get_service_name(),
                error_messages: service_errors,
            });
        }
    }

    // Record all services that failed some number of attempts
    // Return the error codes
    // Front-end will have to handle failures
    Ok(Json(UserCreationResult { user_id, successful_services, failed_services }))
}


pub struct UserUpdate {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub services: Vec<String>,
    pub archived: String,
}

pub struct UserUpdateResult {
    pub changed_password: bool,
}

pub async fn update_user(
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserUpdate>,
) -> Result<Json<UserUpdateResult>, APIError> {
    if let Some(user) = database::get_user(&state.db_pool, &user_id).await? {
        // let changed_password = user.password_hash_salt != salted_password;
        database::update_user(&state.db_pool, &user_id, &payload).await?;
        
    } else {
        return Err(APIError { error_string: format!("Provided user id '{}' does not exist.", user_id) });
    }


    // Once update user completes, mark all services as invalid if the password changed
    todo!()
}

pub async fn get_user(Path(user_id): Path<Uuid>) {
    todo!()
}

pub async fn get_users(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<User>>, APIError> {
    Ok(Json(database::get_users(&state.db_pool).await?))
}

pub async fn create_user_service(Path((user_id, service_name)): Path<(Uuid, String)>) {
    todo!()
}

pub async fn get_user_service(Path((user_id, service_name)): Path<(Uuid, String)>) {
    todo!()
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserServiceSync {
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
    Json(payload): Json<UserServiceSync>
) {
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