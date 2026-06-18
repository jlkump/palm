pub mod db {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub salted_pass: String,
    }
}