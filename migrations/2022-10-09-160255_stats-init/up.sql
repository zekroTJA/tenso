-- Your SQL goes here

-- id -> Primary key
-- link_id -> links.id
-- created_date
-- user_agent

CREATE TABLE stats (
    "id"            VARCHAR(20)     NOT NULL,
    "link_id"       VARCHAR(20)     NOT NULL,
    "created_date"  TIMESTAMP       NOT NULL,
    "user_agent"    TEXT            DEFAULT '',

    PRIMARY KEY ("id"),
    FOREIGN KEY ("link_id")
        REFERENCES "links" ("id")
            ON DELETE CASCADE
)