-- Add migration script here
-- This script will be executed in sqlite database when the migration is run

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS registry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    cert_file TEXT NOT NULL,
    key_file TEXT NOT NULL,
    status TEXT NOT NULL, -- active, inactive
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);