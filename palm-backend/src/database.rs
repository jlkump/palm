use postgres::Client;

use serde::{Deserialize, Serialize};

use crate::route::{UserCreation, UserDelete, UserSync};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash_salt: String,
}

// NOTE:
//   Should we just pass in the database credentials instead and have this method return the Client connection?
//   Either way, Database client is initialized in main, before starting up the API listeners.
pub async fn initialize_database(client: &mut Client) {
    // Question:
    //   Is this the best way to initialize the database?
    //   I think there is a way to simply have a database startup with a schema.
    // - Landon
    client.execute("
        CREATE TABLE Users (
            user_id             INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
            email               TEXT NOT NULL,
            first_name          TEXT NOT NULL,
            last_name           TEXT NOT NULL,
            password_hash       TEXT NOT NULL,
            date_pass_modified  TIMESTAMPTZ NOT NULL,
            archived            BOOLEAN NOT NULL DEFAULT false
        );

        CREATE INDEX Idx_Users_DatePass ON Users (date_pass_modified);
        CREATE INDEX Idx_Users_Archived ON Users USING HASH (archived);

        CREATE TABLE Services (
            service_id          INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
            service_displayname TEXT NOT NULL
        );

        CREATE TABLE UserServices (
            -- Restrict ensures that we can not delete a user if they have a service registered
            -- Likewise, we can not delete a service if any users has the service
            user_id             INT NOT NULL REFERENCES Users (user_id) ON DELETE RESTRICT,
            service_id          INT NOT NULL REFERENCES Services (service_id) ON DELETE RESTRICT,
            service_user_id     TEXT NOT NULL,
            date_pass_synced    TIMESTAMPTZ NULL,

            PRIMARY KEY (user_id, service_id)
        );
        CREATE INDEX Idx_UserServices_DatePass ON UserServices (date_pass_synced);
        CREATE INDEX Idx_UserServices_PendingSync ON UserServices (service_id) WHERE date_pass_synced IS NULL;

        CREATE TABLE UserSessions (
            session_hash  CHAR(64) PRIMARY KEY,
            user_id       INT NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
            issued_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
            expires_at    TIMESTAMPTZ NOT NULL,
            last_seen     TIMESTAMPTZ NOT NULL DEFAULT now()
        );
        CREATE INDEX ON Idx_UserSessions (user_id);
        CREATE INDEX ON Idx_UserSessions (expires_at);
    ", &[]).expect("Failed to initialize database");
}

// TODO: Properly error handle rather than use unwraps
pub async fn create_user(client: &mut Client, info: &UserCreation) {
    client.execute("
        INSERT INTO Users (email, first_name, last_name, pass) 
        VALUES ($1, $2, $3, $4);
    ", &[&info.email, &info.first_name, &info.last_name, &info.password]).unwrap();
}

pub async fn sync_user(client: &mut Client, info: &UserSync) {
    client.execute("
        UPDATE Users
        SET  email = $1, first_name = $2, last_name = $3, pass = $4, archived = $5, 
        WHERE user_id = $6
    ", &[&info.email, &info.first_name, &info.last_name, &info.password, &Box::new(info.archived), &info.service_user_id]).unwrap();
}

pub async fn remove_user(client: &mut Client, info: &UserDelete) {
    client.execute("
        DELETE FROM Users
        WHERE $1
    ", &[&info.service_user_id]).unwrap();
}