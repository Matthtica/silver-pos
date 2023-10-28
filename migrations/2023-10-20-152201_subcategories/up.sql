-- Your SQL goes here
CREATE TABLE subcategories (
  id SERIAL PRIMARY KEY,
  cat_id INT NOT NULL,
  name TEXT NOT NULL
);
