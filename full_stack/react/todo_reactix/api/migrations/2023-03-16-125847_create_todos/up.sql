CREATE TABLE todos (
  id TEXT PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE
)