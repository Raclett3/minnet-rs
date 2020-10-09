CREATE TABLE users (
    id VARCHAR(16) NOT NULL PRIMARY KEY,
    username VARCHAR(256) NOT NULL,
    host VARCHAR(256),
    nickname VARCHAR(256) NOT NULL,
    uri VARCHAR(256) UNIQUE,
    inbox VARCHAR(256),
    public_key VARCHAR(2048)
);

CREATE TABLE users_auth (
    username VARCHAR(64) NOT NULL PRIMARY KEY,
    encrypted_password VARCHAR(256) NOT NULL,
    users_id VARCHAR(16) NOT NULL,
    private_key VARCHAR(2048) NOT NULL,
    public_key VARCHAR(2048) NOT NULL,
    FOREIGN KEY (users_id) REFERENCES users(id)
);
