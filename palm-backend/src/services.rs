use std::collections::HashMap;

use postgres::Client;

use crate::route::{UserCreation, UserDelete, UserSync};

mod forgejo;
mod taiga;

/// API Requests are stateless, thus we don't hold any internal state
/// (This is why all methods are not mutable on self)
pub enum Service {
    Forgejo,
    Taiga,
}

impl Service {
    pub fn get_service_name(&self) -> String {
        todo!()
    }

    pub async fn create_user(&self, data: UserCreation) {
        match self {
            Service::Forgejo => forgejo::create_user(data).await,
            Service::Taiga => taiga::create_user(data).await,
        }
        todo!()
    }

    pub async fn delete_user(&self, data: UserDelete) {
        todo!()
    }

    pub async fn sync_user(&self, data: UserSync) {
        todo!()
    }
}

/// This is run when the backend starts up.
/// 
/// Ensures the creation of all services in the database.
/// Gets the data mapping from name to service info and handlers.
pub async fn create_service_list(db: &mut Client) -> HashMap<String, Service> {
    todo!()
}