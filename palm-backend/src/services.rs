use serde::{Deserialize, Serialize};

use crate::route::{UserCreation, UserDelete, UserServiceSync};

mod forgejo;
mod taiga;

/// API Requests are stateless, thus we don't hold any internal state
/// (This is why all methods are not mutable on self)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    Forgejo,
    Taiga,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceError {
    ForgejoErr(forgejo::ForgejoError),
    TaigaErr(taiga::TaigaError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ServiceError::ForgejoErr(forgejo_error) => &forgejo_error.error_string,
            ServiceError::TaigaErr(taiga_error) => &taiga_error.error_string,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserCreationResult {
    Forgejo(forgejo::UserCreationResult),
    Taiga(taiga::UserCreationResult),
}

impl Service {
    pub fn get_services_from_names(service_names: &Vec<String>) -> Vec<Service> {
        service_names.iter().filter_map(|name| {
            Self::get_service_from_name(name)
        }).collect()
    }

    pub fn get_service_from_name(name: &str) -> Option<Self> {
        match name {
            "Forgejo" => Some(Service::Forgejo),
            "Taiga" => Some(Service::Taiga),
            _ => None,
        }
    }

    pub fn get_service_name(&self) -> String {
        match self {
            Service::Forgejo => "Forgejo",
            Service::Taiga => "Taiga",
        }.to_string()
    }

    pub async fn create_user(&self, data: &UserCreation) -> Result<UserCreationResult, ServiceError> {
        Ok(match self {
            Service::Forgejo => forgejo::create_user(data).await?.into(),
            Service::Taiga => taiga::create_user(data).await?.into(),
        })
    }

    pub async fn delete_user(&self, data: &UserDelete) {
        match self {
            Service::Forgejo => forgejo::delete_user(data).await,
            Service::Taiga => taiga::delete_user(data).await,
        }
    }

    pub async fn sync_user(&self, data: &UserServiceSync) {
        match self {
            Service::Forgejo => forgejo::sync_user(data).await,
            Service::Taiga => taiga::sync_user(data).await,
        }
    }
}