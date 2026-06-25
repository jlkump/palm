pub mod service {
    use crate::model::intermediate::{UserCreation, UserDelete, UserSync};

    pub struct Error;
    pub struct CreationError;
    pub struct SyncError;
    pub struct RemoveError;
    
    pub trait Service {
        fn create_user(&mut self, info: &UserCreation) -> impl Future;
        fn sync_user(&mut self, info: &UserSync) -> impl Future;
        fn remove_user(&mut self, info: &UserDelete)  -> impl Future;
    }
}

pub mod db {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct User {
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub password_hash_salt: String,
    }
}

pub mod intermediate {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserCreation {
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub password: String,
    }

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserSync {
        pub service_user_id: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub password: String,
        pub archived: bool,
    }

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UserDelete {
        pub service_user_id: String,
    }
}