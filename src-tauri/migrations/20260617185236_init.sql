-- Add migration script here
CREATE TABLE IF NOT EXISTS loans (
    id BLOB PRIMARY KEY,
    date_created TEXT NOT NULL,
    date_updated TEXT NOT NULL,
    principal TEXT NOT NULL,
    rate TEXT NOT NULL,
    number_of_months TEXT NOT NULL,
    monthly_payment TEXT NOT NULL,
);