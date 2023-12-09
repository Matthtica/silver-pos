-- Add migration script here

DROP TABLE IF EXISTS cashflow;

CREATE TABLE cashflow (
  id SERIAL PRIMARY KEY,
  time TIMESTAMP NOT NULL,
  amount INTEGER NOT NULL,
  description TEXT NOT NULL
)
