-- Add up migration script here
CREATE TABLE top_timezone (
    country VARCHAR(255) PRIMARY KEY,
    amount INTEGER NOT NULL DEFAULT 0
);