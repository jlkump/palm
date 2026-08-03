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
    }

    pub async fn delete_user(&self, data: UserDelete) {
        match self {
            Service::Forgejo => forgejo::delete_user(data).await,
            Service::Taiga => taiga::delete_user(data).await,
        }
    }

    pub async fn sync_user(&self, data: UserSync) {
        match self {
            Service::Forgejo => forgejo::sync_user(data).await,
            Service::Taiga => taiga::sync_user(data).await,
        }
    }
}