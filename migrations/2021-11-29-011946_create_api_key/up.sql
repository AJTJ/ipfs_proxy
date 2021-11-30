-- Your SQL goes here
CREATE TABLE api_key (
  id SERIAL PRIMARY KEY,
  user_id SERIAL NOT NULL,
  key_value uuid NOT NULL,
  is_enabled boolean NOT NULL DEFAULT '1',
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
)