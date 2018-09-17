CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR UNIQUE NOT NULL,
  referral_code VARCHAR UNIQUE NOT NULL,
  referrer_id INTEGER,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP
);
