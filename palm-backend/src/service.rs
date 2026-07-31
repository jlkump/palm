use crate::route::{UserCreation, UserDelete, UserSync};

pub struct Error;
pub struct CreationError;
pub struct SyncError;
pub struct RemoveError;

pub trait Service {
    /// Used to identify a service uniquely
    fn get_service_name() -> String;

    fn create_user(data: UserCreation) -> impl Future;
    fn delete_user(data: UserDelete) -> impl Future;
    fn sync_user(data: UserSync) -> impl Future;
}

/// This is run when the backend starts up.
/// 
/// Ensures the creation of all services in the database.
/// Gets the data mapping from name to service info and handlers.
pub async fn create_service_list(db: Client) -> Vec<dyn Service> {
    todo!()
}