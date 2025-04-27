-- Your SQL goes here
CREATE TABLE voter (
    id                  INTEGER Not NULL PRIMARY KEY AUTOINCREMENT,
    verifier_id INTEGER NOT NULL,
    proof_type          TEXT NOT NULL,
    utc_timestamp       BIGINT NOT NULL,
    offset              INTEGER NOT NULL,
    voter_pubkey        BLOB(32)  NOT NULL UNIQUE,
    version             SMALLINT Not NULL,
    verifier_sig        BLOB(65)  NOT NULL,
    voter_info          TEXT,

    FOREIGN KEY (verifier_id) REFERENCES verifier(id)
);

-- CREATE INDEX idx_verifier_name ON verifier(name);

