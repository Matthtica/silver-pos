-- Your SQL goes here

CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  code_name VARCHAR NOT NULL,
  amount INT NOT NULL,
  price INT NOT NULL,
  cat_id INT NOT NULL
);
