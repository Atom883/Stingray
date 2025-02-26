CREATE TABLE users (
    id TEXT PRIMARY KEY,
    password_bcrypt TEXT NOT NULL,
    created_at TEXT NOT NULL
);
