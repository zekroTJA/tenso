-- Your SQL goes here

CREATE TABLE auth (
    "username" VARCHAR(32) NOT NULL,
    "password_hash" VARCHAR NOT NULL,

    PRIMARY KEY ("username")
);