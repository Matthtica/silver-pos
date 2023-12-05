-- Your SQL goes here
CREATE TABLE tmp AS SELECT * FROM categories;

DROP TABLE categories;

CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  code_name VARCHAR NOT NULL,
  color VARCHAR NOT NULL,
  icon VARCHAR NOT NULL
);

INSERT INTO categories SELECT id, name, code_name, color, icon FROM tmp;

DROP TABLE tmp;
