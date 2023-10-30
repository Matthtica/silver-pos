-- Your SQL goes here
CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE categories_tmp AS SELECT * FROM categories;

ALTER TABLE categories_tmp ADD color TEXT NOT NULL DEFAULT '#ffffff';

INSERT INTO categories_tmp (id, name, new_col) SELECT id, name, DEFAULT FROM categories;

DROP TABLE categories;

ALTER TABLE categories_tmp RENAME TO categories;
