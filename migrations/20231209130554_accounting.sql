-- Add migration script here

CREATE TABLE cashflow (
  id SERIAL PRIMARY KEY,
  timestemp TIMESTAMP NOT NULL,
  amount INTEGER NOT NULL,
  description TEXT NOT NULL
)
