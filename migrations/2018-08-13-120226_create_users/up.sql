-- Your SQL goes here

CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NULL,
  github_oauth_token TEXT NULL,
  external_id BIGINT NOT NULL,
  site_admin BOOLEAN NOT NULL DEFAULT FALSE
)
