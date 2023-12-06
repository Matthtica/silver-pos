-- Add migration script here

DROP TABLE vouchers;

CREATE TABLE vouchers (
  id SERIAL PRIMARY KEY,
  voucher_id VARCHAR NOT NULL,
  customer_name VARCHAR,
  customer_contact VARCHAR,
  item_ids INTEGER[] NOT NULL,
  item_quantities INTEGER[] NOT NULL,
  item_prices INTEGER[] NOT NULL,
  time TIMESTAMP NOT NULL,
  status BOOLEAN NOT NULL DEFAULT TRUE
);
