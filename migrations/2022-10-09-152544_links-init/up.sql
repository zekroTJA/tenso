-- Your SQL goes here

-- id               -> primary key
-- ident
-- creator          -> auth.username
-- created_date
-- destination
-- enabled
-- permanent_redirect

CREATE TABLE links (
    "id"                    VARCHAR(20)     NOT NULL,
    "ident"                 TEXT            NOT NULL,
    "creator_id"            VARCHAR(32)     NOT NULL,
    "created_date"          TIMESTAMP       NOT NULL,
    "destination"           TEXT            NOT NULL,
    "enabled"               BOOLEAN         NOT NULL DEFAULT 'false',
    "permanent_redirect"    BOOLEAN         NOT NULL DEFAULT 'false',

    PRIMARY KEY ("id"),
    FOREIGN KEY ("creator_id")
        REFERENCES "auth" ("username")
            ON DELETE CASCADE
)