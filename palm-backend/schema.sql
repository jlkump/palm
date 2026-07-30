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