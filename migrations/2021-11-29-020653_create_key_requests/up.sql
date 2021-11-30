-- Your SQL goes here
CREATE TABLE key_requests (
  id SERIAL PRIMARY KEY,
  api_key_id SERIAL NOT NULL,
  date_time timestamp without time zone NOT NULL,
  FOREIGN KEY (api_key_id) REFERENCES api_key(id) ON DELETE CASCADE
)