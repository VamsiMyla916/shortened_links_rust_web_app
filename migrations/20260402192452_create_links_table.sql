-- Add migration script here
CREATE TABLE links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_url TEXT NOT NULL,
    rusty_alias TEXT NOT NULL UNIQUE
);