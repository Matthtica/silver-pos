-- Your SQL goes here
CREATE TABLE transections (
  id SERIAL PRIMARY KEY,
  direction BOOLEAN NOT NULL DEFAULT FALSE,
  time TIMESTAMP NOT NULL,
  price INT NOT NULL,
  cat_id INT NOT NULL,
  subcat_id INT NOT NULL
);
