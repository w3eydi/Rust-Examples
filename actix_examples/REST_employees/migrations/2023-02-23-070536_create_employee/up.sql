-- Your SQL goes here
CREATE TABLE employees (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name VARCHAR(140) NOT NULL,
  last_name VARCHAR(140) NOT NULL,
  department VARCHAR(140) NOT NULL,
  salary INT NOT NULL,
  age INT NOT NULL
)
