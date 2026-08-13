use serde::{Deserialize, Serialize};

use crate::{route::{UserCreation, UserDelete, UserServiceSync}, services::{ServiceError}};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaigaError {
    pub error_string: String,
}

impl From<TaigaError> for ServiceError {
    fn from(value: TaigaError) -> Self {
        ServiceError::TaigaErr(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreationResult {
    pub user_id: String,
}

impl From<UserCreationResult> for super::UserCreationResult {
    fn from(value: UserCreationResult) -> Self {
        super::UserCreationResult::Taiga(value)
    }
}
pub async fn create_user(data: &UserCreation) -> Result<UserCreationResult, TaigaError> {
    todo!()
}

pub async fn delete_user(data: &UserDelete) {
    todo!()
}

pub async fn sync_user(data: &UserServiceSync) {
    todo!()
}