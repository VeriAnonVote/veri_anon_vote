-- Your SQL goes here
CREATE TABLE vote_record (
    id                  INTEGER Not NULL PRIMARY KEY AUTOINCREMENT,
    vote_choice         TEXT NOT NULL,
    ring_sig            BLOB NOT NULL
    -- vote_choice         SMALLINT NOT NULL,
    -- utc_timestamp       BIGINT NOT NULL,
    -- offset              INTEGER NOT NULL,
    -- version             SMALLINT Not NULL,
);

-- CREATE INDEX idx_verifier_name ON verifier(name);

