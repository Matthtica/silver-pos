-- Your SQL goes here
CREATE TABLE tmp AS SELECT * FROM categories;

DROP TABLE categories;

CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL DEFAULT 'Unnamed',
  m_name TEXT NOT NULL DEFAULT 'အမည်မရှိ',
  code_name VARCHAR NOT NULL DEFAULT 'None',
  color VARCHAR NOT NULL,
  icon VARCHAR NOT NULL
);

INSERT INTO categories SELECT id, name, n_name, code_name, color, icon FROM tmp;
DROP TABLE tmp;
