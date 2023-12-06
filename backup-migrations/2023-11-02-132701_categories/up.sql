-- Your SQL goes here
CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  color VARCHAR NOT NULL,
  icon VARCHAR NOT NULL
);
