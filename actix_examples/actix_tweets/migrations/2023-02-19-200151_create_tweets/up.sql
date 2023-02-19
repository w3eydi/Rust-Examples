-- Your SQL goes here
CREATE TABLE tweets (
  id SERIAL NOT NULL PRIMARY KEY,
  message VARCHAR(140) NOT NULL,
  created_at TIMESTAMP NOT NULL
)