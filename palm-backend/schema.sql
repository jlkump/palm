CREATE TABLE Users (
    user_id             INT PRIMARY KEY,
    email               VARCHAR(510) NOT NULL,
    first_name          VARCHAR(255) NOT NULL,
    last_name           VARCHAR(255) NOT NULL,
    pass                VARCHAR(255) NOT NULL,
    date_pass_modified  DATETIME NOT NULL,
    archived            BOOLEAN NOT NULL,
);
CREATE INDEX Idx_Users_DatePass ON Users (date_pass_modified);
CREATE INDEX Idx_Users_Archived ON Users USING HASH (archived);

CREATE TABLE Services (
    service_id          INT PRIMARY KEY,
    service_displayname VARCHAR(255) NOT NULL,
);

CREATE TABLE UserServices (
    -- Restrict ensures that we can not delete a user if they have a service registered
    -- Likewise, we can not delete a service if any users has the service
    user_id             INT NOT NULL REFERENCES Users (user_id) ON DELETE RESTRICT,
    service_id          INT NOT NULL REFERENCES Services (service_id) ON DELETE RESTRICT,
    service_user_id     VARCHAR(255) NOT NULL,
    date_pass_synced    DATETIME NULL,

    PRIMARY KEY (user_id, service_id),
);
CREATE INDEX Idx_UserServices_DatePass ON UserServices (date_pass_synced);