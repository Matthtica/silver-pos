-- Your SQL goes here

ALTER TABLE items ADD COLUMN m_name VARCHAR NOT NULL DEFAULT 'အမည်မရှိ';

CREATE TABLE tmp (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL DEFAULT 'unnamed',
  m_name VARCHAR NOT NULL DEFAULT 'အမည်မရှိ',
  code_name VARCHAR NOT NULL,
  amount INT NOT NULL,
  price INT NOT NULL,
  cat_id INT NOT NULL
);

INSERT INTO tmp SELECT id, name, m_name, code_name, amount, price, cat_id FROM items;

DROP TABLE items;

ALTER TABLE tmp RENAME TO items;
