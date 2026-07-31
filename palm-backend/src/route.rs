use axum::{Json, response::Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};


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
pub async fn create_user(Json(payload): Json<UserCreation>) -> Json<UserCreation> {
    // First attempt creation of the Palm user
    //      Return any error codes

    // Then, attempt creation across all the services.
    
    Json(payload)
}

pub async fn create_user_service() {

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

pub async fn sync_user_service(Json(payload): Json<UserSync>) -> Json<UserSync> {
    Json(payload)
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserDelete {
    pub service_user_id: String,
}