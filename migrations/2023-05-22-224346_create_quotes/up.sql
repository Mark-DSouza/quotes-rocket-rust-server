-- Your SQL goes here
CREATE TABLE quotes (
  id SERIAL PRIMARY KEY,
  author VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  category VARCHAR NOT NULL
)