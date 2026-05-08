-- Users
CREATE TABLE users (
    id            TEXT    PRIMARY KEY,                                -- UUID v7
    username      TEXT    NOT NULL UNIQUE,
    full_name     TEXT    NOT NULL,
    role          TEXT    NOT NULL CHECK (role IN ('admin','bookkeeper','partner_view')),
    password_hash TEXT    NOT NULL,                                   -- PHC Argon2id
    is_active     INTEGER NOT NULL DEFAULT 1,
    created_at    TEXT    NOT NULL,
    updated_at    TEXT    NOT NULL
);
CREATE INDEX idx_users_username ON users(username);

-- Entities (filled in Feature 2)
CREATE TABLE entities (
    id         TEXT    PRIMARY KEY,
    code       TEXT    NOT NULL UNIQUE,        -- ECE / ECEMCO / ECSL / ECH / ECT / ESP
    name       TEXT    NOT NULL,
    is_billing INTEGER NOT NULL DEFAULT 0,     -- 1 for ECE
    created_at TEXT    NOT NULL,
    updated_at TEXT    NOT NULL
);

-- Tamper-evident audit chain
CREATE TABLE audit_log (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    record_id    TEXT    NOT NULL,             -- UUID v7
    occurred_at  TEXT    NOT NULL,             -- ISO8601 UTC
    actor_id     TEXT,                         -- nullable for system events
    action       TEXT    NOT NULL,             -- e.g. 'auth.login', 'closeout.finalize'
    entity_id    TEXT,
    payload_json TEXT    NOT NULL,             -- canonical JSON of the event
    prev_hash    BLOB    NOT NULL,             -- 32 bytes; zeroes for genesis
    hash         BLOB    NOT NULL              -- SHA-256(prev_hash || canonical-record)
);
CREATE INDEX idx_audit_action   ON audit_log(action);
CREATE INDEX idx_audit_actor    ON audit_log(actor_id);
CREATE INDEX idx_audit_occurred ON audit_log(occurred_at);

-- Lightweight key/value (schema metadata, install id, etc.)
CREATE TABLE app_meta (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
