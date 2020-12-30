-- Your SQL goes here
CREATE TABLE translations (
  id VARCHAR NOT NULL PRIMARY KEY,
  key VARCHAR NOT NULL,
  target TEXT NOT NULL,
  language VARCHAR NOT NULL
)
