-- Add migration script here

CREATE TABLE IF NOT EXISTS categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  m_name VARCHAR NOT NULL,
  code_name VARCHAR NOT NULL,
  color VARCHAR NOT NULL,
  icon VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  m_name VARCHAR NOT NULL,
  code_name VARCHAR NOT NULL,
  amount INT NOT NULL,
  price INT NOT NULL,
  cat_id INT NOT NULL
);
