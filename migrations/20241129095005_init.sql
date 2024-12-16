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
    host TEXT NOT NULL DEFAULT 'localhost',
    port INTEGER NOT NULL,
    username TEXT,
    password TEXT,
    cert_file TEXT,
    key_file TEXT,
    status TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);