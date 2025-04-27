-- Your SQL goes here
CREATE TABLE verifier (
    id                  INTEGER Not NULL PRIMARY KEY AUTOINCREMENT,
    name                TEXT NOT NULL,
    wallet_address      BLOB(20)  NOT NULL UNIQUE,
    max_upload_count    INTEGER NOT NULL DEFAULT 10000,
    api_key             TEXT NOT NULL UNIQUE CHECK (LENGTH(api_key) >= 128),
    description         TEXT
);

CREATE INDEX idx_verifier_name ON verifier(name);

