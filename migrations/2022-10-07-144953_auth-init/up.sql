-- Your SQL goes here

CREATE TABLE auth (
    "username" VARCHAR(32) PRIMARY KEY,
    "password_hash" VARCHAR NOT NULL
);