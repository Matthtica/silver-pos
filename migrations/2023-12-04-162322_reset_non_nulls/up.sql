-- Your SQL goes here
DROP TABLE IF EXISTS vouchers;

CREATE TABLE vouchers (
  id SERIAL PRIMARY KEY,
  voucher_id VARCHAR NOT NULL,
  customer_name VARCHAR,
  customer_contact VARCHAR,
  cart_items cart_item[] NOT NULL,
  time TIMESTAMP NOT NULL,
  status BOOLEAN NOT NULL DEFAULT TRUE
);
