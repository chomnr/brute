-- Add up migration script here
CREATE TABLE top_ip (
    ip VARCHAR(15) PRIMARY KEY,
    amount INTEGER NOT NULL DEFAULT 0
);