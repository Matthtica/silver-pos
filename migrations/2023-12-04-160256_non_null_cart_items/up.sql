-- Your SQL goes here

DROP TABLE vouchers;

CREATE DOMAIN non_null_cart_item_array AS cart_item[] DEFAULT '{}'::cart_item[] NOT NULL;
CREATE TABLE vouchers (
  id SERIAL PRIMARY KEY,
  voucher_id VARCHAR NOT NULL,
  customer_name VARCHAR,
  customer_contact VARCHAR,
  cart_items non_null_cart_item_array,
  time TIMESTAMP NOT NULL,
  status BOOLEAN NOT NULL DEFAULT TRUE
);
